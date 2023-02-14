//! GPU Goldilocks NTT implementation
//!
//! See the [wgpu compute example](https://github.com/gfx-rs/wgpu/blob/bb01d723ba90654fdec85d931edbe7c9be56869e/wgpu/examples/hello-compute/main.rs).
//!
//! TODO: SHA3: <https://ieeexplore.ieee.org/document/9585122>
//! <https://github.com/ethereum-mining/ethminer/blob/8eaf50ab3baad9a1747e04dc295793d46740dcae/libethash-cl/kernels/cl/ethash.cl#L111-L175>
#![cfg(feature = "gpu")]

use pollster::FutureExt;
use std::{borrow::Cow, mem::size_of, str::FromStr};
use wgpu::util::DeviceExt;

use crate::{ntt::Ntt, utils::div_round_up, Field};

// Indicates a u32 overflow in an intermediate Collatz value
const OVERFLOW: u32 = 0xffffffff;

const SHADER: wgpu::ShaderModuleDescriptor = wgpu::ShaderModuleDescriptor {
    label:  Some("shader.wgsl"),
    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(concat!(
        // Concatenate source files as a primitive form of preprocessor.
        include_str!("utils.wgsl"),
        include_str!("goldilocks.wgsl"),
        include_str!("ntt.wgsl")
    ))),
};

struct GpuNtt {
    gpu:        Gpu,
    size:       usize,
    count:      usize,
    shader:     wgpu::ShaderModule,
    pipeline:   wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    input:      wgpu::Buffer,
    output:     wgpu::Buffer,
}

struct Gpu {
    instance: wgpu::Instance,
    adapter:  wgpu::Adapter,
    device:   wgpu::Device,
    queue:    wgpu::Queue,
}

impl Gpu {
    async fn new() -> Self {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let info = adapter.get_info();
        eprintln!(
            "Using {:?} {:?} {}",
            info.backend, info.device_type, info.name
        );
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label:    None,
                    features: wgpu::Features::empty(), // MAPPABLE_PRIMARY_BUFFERS ?
                    limits:   wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .unwrap();
        eprintln!(
            "Max workgroup dimension: {:?}",
            device.limits().max_compute_workgroups_per_dimension
        );
        eprintln!(
            "        Max buffer size: {:?}",
            device.limits().max_buffer_size
        );
        eprintln!(
            "       Max binding size: {:?}",
            device.limits().max_storage_buffer_binding_size
        );
        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}

impl GpuNtt {
    fn new(size: usize, count: usize) -> Self {
        assert_eq!(size, 4);
        let buffer_size = (count * size * size_of::<Field>()) as wgpu::BufferAddress;

        let gpu = Gpu::new().block_on();

        let shader = gpu.device.create_shader_module(SHADER);
        let pipeline = gpu
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label:       Some("Compute Pipeline"),
                layout:      None,
                module:      &shader,
                entry_point: "main",
            });

        let input = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label:              Some("Input Buffer"),
            size:               buffer_size,
            usage:              wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label:              Some("Output Buffer"),
            size:               buffer_size,
            usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   None,
            layout:  &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: input.as_entire_binding(),
            }],
        });

        Self {
            gpu,
            size,
            count,
            shader,
            pipeline,
            bind_group,
            input,
            output,
        }
    }

    fn dispatch_size(n: usize) -> (u32, u32, u32) {
        if n < 65535 {
            return (n as u32, 1, 1);
        } else {
            (65535, std::cmp::max(1, (n / 65535) as u32), 1)
        }
    }
}

impl Ntt for GpuNtt {
    fn len(&self) -> usize {
        self.size
    }

    fn ntt(&self, values: &mut [Field]) {
        assert_eq!(values.len(), self.size * self.count);

        // Command sequence
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Buffer"),
            });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
            });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.insert_debug_marker("Compute NTT");

            // Number of cells to run, the (x,y,z) size of item being processed
            let (x, y, z) = Self::dispatch_size(self.count);
            cpass.dispatch_workgroups(x, y, 1);
        }

        // Submits command encoder for processing
        let data = bytemuck::cast_slice(values);
        self.gpu.queue.write_buffer(&self.input, 0, &data);
        encoder.copy_buffer_to_buffer(&self.input, 0, &self.output, 0, self.input.size());
        let submission_id = self.gpu.queue.submit(Some(encoder.finish()));

        // Set up buffer for mapped reading
        let output_slice = self.output.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        output_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Wait for buffer to be ready (blocking)
        // let now = std::time::Instant::now();
        self.gpu
            .device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission_id));
        // let duration = now.elapsed();
        // dbg!(duration.as_secs_f64());
        receiver.receive().block_on().unwrap().unwrap();

        // Read data back from buffer
        let data = output_slice.get_mapped_range();
        values.copy_from_slice(bytemuck::cast_slice(&data));

        // Unmap buffer
        drop(data);
        self.output.unmap();
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt, *};

    #[test]
    fn test_gpu_ntt() {
        test_ntt(GpuNtt::new(4, 1));
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use crate::ntt::MIN_WORK_SIZE;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "gpu", GpuNtt::new(4, MIN_WORK_SIZE / 4));
    }
}

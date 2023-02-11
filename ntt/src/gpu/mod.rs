//! GPU Golilocks NTT implementation
//! 
//! See the [wgpu compute example](https://github.com/gfx-rs/wgpu/blob/bb01d723ba90654fdec85d931edbe7c9be56869e/wgpu/examples/hello-compute/main.rs).
#![cfg(feature = "gpu")]

use std::{borrow::Cow, str::FromStr};
use wgpu::util::DeviceExt;

// Indicates a u32 overflow in an intermediate Collatz value
const OVERFLOW: u32 = 0xffffffff;

const SHADER: wgpu::ShaderModuleDescriptor = wgpu::ShaderModuleDescriptor {
    label:  Some("shader.wgsl"),
    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
};

struct GpuNtt {
    gpu: Gpu,

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
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label:    None,
                    features: wgpu::Features::empty(),
                    limits:   wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .unwrap();
        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}

impl GpuNtt {
    async fn new() -> Self {
        let size = 8;

        let gpu = Gpu::new().await;

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
            label: Some("Input Buffer"),
            size,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
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
            shader,
            pipeline,
            bind_group,
            input,
            output,
        }
    }

    fn run(&mut self, values: &mut [u32; 2]) {
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoded"),
            });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.insert_debug_marker("Compute NTT");
            // Number of cells to run, the (x,y,z) size of item being processed
            cpass.dispatch_workgroups(2, 1, 1);
        }

        // Submits command encoder for processing
        self.gpu
            .queue
            .write_buffer(&self.input, 0, bytemuck::cast_slice(values));
        encoder.copy_buffer_to_buffer(&self.input, 0, &self.output, 0, self.input.size());
        self.gpu.queue.submit(Some(encoder.finish()));

        // Set up buffer for mapped reading
        let output_slice = self.output.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        output_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Wait for buffer to be ready (blocking)
        self.gpu.device.poll(wgpu::Maintain::Wait);
        pollster::block_on(receiver.receive()).unwrap();

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
    use super::*;

    #[test]
    fn test_gpu() {
        pollster::block_on(run());
    }

    #[test]
    fn test_gpu_ntt() {
        let mut gpu_ntt = pollster::block_on(GpuNtt::new());

        let mut values = [3, 27];
        gpu_ntt.run(&mut values);
        dbg!(values);
    }
}

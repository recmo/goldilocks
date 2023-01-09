use rayon::prelude::*;
use std::{borrow::Cow, str::FromStr, time::Instant};
use wgpu::util::DeviceExt;
use plonky2_field::goldilocks_field::GoldilocksField;

fn fib(n_base: u64) -> u64 {
    let mut n = n_base;
    let mut a = GoldilocksField(0);
    let mut b = GoldilocksField(1);
    loop {
        if n <= 1 {
            break;
        }
        let t = a + b;
        a = b;
        b = t;
        n -= 1;
    }
    b.0
}

fn fac(n: u64) -> u64 {
    let mut r = GoldilocksField(1);
    for i in 2..=n {
        r *= GoldilocksField(i);
    }
    r.0
}

async fn run_cpu(numbers: &[u64]) -> Vec<u64> {
    numbers.iter().copied().map(fac).collect()
}

async fn run_cpu_par(numbers: &[u64]) -> Vec<u64> {
    numbers.par_iter().copied().map(fac).collect()
}

struct Gpu {
    instance:          wgpu::Instance,
    adapter:           wgpu::Adapter,
    device:            wgpu::Device,
    queue:             wgpu::Queue,
    cs_module:         wgpu::ShaderModule,
    staging_buffer:    wgpu::Buffer,
    storage_buffer:    wgpu::Buffer,
    compute_pipeline:  wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group:        wgpu::BindGroup,
}

impl Gpu {
    async fn new(size: usize) -> Self {
        // Instantiates instance of WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // `request_adapter` instantiates the general connection to the GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        // `request_device` instantiates the feature specific connection to the GPU,
        // defining some parameters,  `features` being the available features.
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

        

        // Loads the shader from WGSL
        let mut source = include_str!("../shader/goldilocks.wgsl").to_string();
        source.push_str(include_str!("../shader/mul.wgsl"));
        
        let cs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label:  None,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let size = size as wgpu::BufferAddress;
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_WRITE
                | wgpu::BufferUsages::MAP_READ
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Instantiates buffer.
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a
        // shader).   The destination of a copy.
        //   The source of a copy.
        let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // A bind group defines how buffers are accessed by shaders.
        // It is to WebGPU what a descriptor set is to Vulkan.
        // `binding` here refers to the `binding` of a buffer in the shader (`layout(set
        // = 0, binding = 0) buffer`).

        // A pipeline specifies the operation of a shader

        // Instantiates the pipeline.
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label:       None,
            layout:      None,
            module:      &cs_module,
            entry_point: "main",
        });

        // Instantiates the bind group, once again specifying the binding of buffers.
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   None,
            layout:  &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        Self {
            instance,
            adapter,
            device,
            queue,
            cs_module,
            staging_buffer,
            storage_buffer,
            compute_pipeline,
            bind_group_layout,
            bind_group,
        }
    }

    async fn run(&mut self, numbers: &[u64]) -> Vec<u64> {
        // Gets the size in bytes of the buffer.
        let slice_size = numbers.len() * std::mem::size_of::<u64>();
        let size = slice_size as wgpu::BufferAddress;

        self.queue
            .write_buffer(&self.staging_buffer, 0, bytemuck::cast_slice(&numbers));

        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Sets adds copy operation to command encoder.
        // Will copy data from staging buffer on GPU to storage buffer on CPU.
        encoder.copy_buffer_to_buffer(&self.staging_buffer, 0, &self.storage_buffer, 0, size);

        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&self.compute_pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[]);
        cpass.dispatch(numbers.len() as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        drop(cpass);

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(&self.storage_buffer, 0, &self.staging_buffer, 0, size);

        // Submits command encoder for processing
        let commands = encoder.finish();

        self.queue.submit(Some(commands));

        // Note that we're not calling `.await` here.
        let buffer_slice = self.staging_buffer.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);

        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        self.device.poll(wgpu::Maintain::Wait);

        // Awaits until `buffer_future` can be read from
        if let Ok(()) = buffer_future.await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            self.staging_buffer.unmap(); // Unmaps buffer from memory
                                         // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                         //   delete myPointer;
                                         //   myPointer = NULL;
                                         // It effectively frees the memory

            // Returns data from buffer
            result
        } else {
            panic!("failed to run compute on gpu!")
        }
    }
}

async fn run() {
    let size = 50000;
    let input = (0 .. size).collect::<Vec<_>>();

    let mut gpu = Gpu::new(input.len() * 8).await;

    println!("Compute using CPU (parallel)");
    let now = Instant::now();
    let steps = run_cpu_par(&input).await;
    let elapsed = now.elapsed();
    println!("Done in {:.2?}", elapsed);

    println!("Compute using GPU");
    let now = Instant::now();
    let steps2 = gpu.run(&input).await;
    let elapsed = now.elapsed();
    println!("Done in {:.2?}", elapsed);

    assert_eq!(steps, steps2);
}

fn main() {
    env_logger::init();
    pollster::block_on(run());
}

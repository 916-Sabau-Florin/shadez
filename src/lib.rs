pub use bytemuck::Pod;


enum Either<L,R> {
    Left(L),
    Right(R)
}

pub struct ComputeShader {
    device: wgpu::Device,
    queue: wgpu::Queue,
    cs_module: wgpu::ShaderModule,
}


impl ComputeShader {
    pub fn from_source<'a>(source: &'a str) -> Self {
        
        let adapter = smol::future::block_on(wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: None,
                },
                wgpu::BackendBit::PRIMARY
        )).unwrap();

        let (device, queue) = smol::future::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions::default(),
                limits: wgpu::Limits::default(),
            }
        ));

        let mut compiler = shaderc::Compiler::new().unwrap();        
        let spirv = compiler.compile_into_spirv(
            source, 
            shaderc::ShaderKind::Compute, 
            "compute_shader", 
            "main", 
            None
        ).unwrap();

        let data = wgpu::read_spirv(std::io::Cursor::new(spirv.as_binary_u8())).unwrap();

        let cs_module = device.create_shader_module(&data);
    
        Self {
            device,
            queue,
            cs_module,
        } 
    }

    pub fn from_spirv<'a>(bytes: &'a [u8]) -> Self {

        let adapter = smol::future::block_on(wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: None,
                },
                wgpu::BackendBit::PRIMARY
        )).unwrap();

        let (device, queue) = smol::future::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions::default(),
                limits: wgpu::Limits::default(),
            }
        ));


        let data = wgpu::read_spirv(std::io::Cursor::new(bytes)).unwrap();

        let cs_module = device.create_shader_module(&data);
    
        Self {
            device,
            queue,
            cs_module,
        } 
    }

    pub fn pass(&self) -> ComputePass {
        ComputePass {
            shader: &self,
            buffer_data: Vec::new(),
            buffers: Vec::new(),
        }
    }

}

pub struct ComputePass<'s,'b> {
    shader: &'s ComputeShader,

    buffer_data: Vec<Either<&'b [u8], &'b mut [u8]>>,
    buffers: Vec<wgpu::Buffer>,

}


impl<'s, 'b> ComputePass<'s, 'b> {

    pub fn bind_buffer<B: bytemuck::Pod>(mut self, buffer: &'b [B]) -> Self {

        let buffer = bytemuck::cast_slice(buffer); 

        self.buffers.push(self.shader.device.create_buffer_with_data(
                buffer,
                wgpu::BufferUsage::STORAGE_READ
        ));

        self.buffer_data.push(Either::Left(buffer));
        self
    }

    pub fn bind_mut_buffer<B: bytemuck::Pod>(mut self, buffer: &'b mut [B]) -> Self { 
        
        let buffer = bytemuck::cast_slice_mut(buffer);

        self.buffers.push(self.shader.device.create_buffer_with_data(
                buffer,
                wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::MAP_READ
        ));

        self.buffer_data.push(Either::Right(buffer));
        self
    }

    pub async fn compute_async(mut self, x: u32, y: u32, z: u32) {

        let mut bind_group_entries: Vec<wgpu::BindGroupLayoutEntry> = Vec::new();
        let mut bindings: Vec<wgpu::Binding> = Vec::new();


        for (idx, either_buffer) in self.buffer_data.iter().enumerate() {
            match either_buffer {
                Either::Left(data) => {
                    bind_group_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: idx as u32,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer {
                            dynamic: false,
                            readonly: true,
                        }
                    });
                    bindings.push(wgpu::Binding {
                        binding: idx as u32,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &self.buffers[idx],
                            range: 0..std::mem::size_of_val(*data) as wgpu::BufferAddress
                        }
                    });
                },
                Either::Right(data) => {
                    bind_group_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: idx as u32,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer { dynamic: false,
                            readonly: false,
                        }
                    });
                    bindings.push(wgpu::Binding {
                        binding: idx as u32,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &self.buffers[idx],
                            range: 0..std::mem::size_of_val(*data) as wgpu::BufferAddress
                        }
                    });
                }            
            }
        }
        
        let bind_group_layout = self.shader.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &bind_group_entries,
            label: Some("uniform_bind_group_layout"),
        });

        let bind_group = self.shader.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &bindings,
            label: Some("bind_group"),
        });

        let pipeline_layout = self.shader.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout]
        });

        let pipeline = self.shader.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: wgpu::ProgrammableStageDescriptor {
                module: &self.shader.cs_module,
                entry_point:"main",
            }
        });

        let mut encoder = self.shader.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command_encoder")
        });


        {
            let mut pass = encoder.begin_compute_pass();
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0,&bind_group, &[]);
            pass.dispatch(x,y,z);
        }

        self.shader.queue.submit(&[encoder.finish()]);
        
 
        for (idx, either_buffer) in self.buffer_data.iter_mut().enumerate() {
            match either_buffer {
                Either::Left(_buffer) => {},
                Either::Right(buffer) => {
                    let future = self.buffers[idx].map_read(0, 
                            std::mem::size_of_val(*buffer) as u64);
                    self.shader.device.poll(wgpu::Maintain::Wait);

                    let data = future.await.unwrap();

                    (*buffer).copy_from_slice(data.as_slice()); 

                    drop(data);
                    self.buffers[idx].unmap();
                }
            }
        } 
    }

    pub fn compute(self, x: u32, y: u32, z: u32) {
        smol::run(self.compute_async(x,y,z)); 
    }

}

use std::iter;
use std::time::{Duration, Instant};

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod shapes;
mod texture;
use shapes::*;
mod vertex;
use vertex::VertexTexture;

enum Shape {
    Square,
    Circle,
    Cross,
}

const VERTICES: &[VertexTexture] = &[
    VertexTexture {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    }, // upper right
    VertexTexture {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
    }, // upper left
    VertexTexture {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    }, // lower right
    VertexTexture {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    }, // lower left
];

const INDICES: &[u16] = &[0, 1, 2, 1, 3, 2];

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    shader: wgpu::ShaderModule,
    pipeline_layout: wgpu::PipelineLayout,
    shader_with_texture: wgpu::ShaderModule,
    pipeline_with_texture_layout: wgpu::PipelineLayout,
    pipeline_with_texture: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    size: winit::dpi::PhysicalSize<u32>,
    bundle: wgpu::RenderBundle,
    rebuild_bundle: bool,
    diffuse_texture: texture::Texture,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_bind_group1: wgpu::BindGroup,
    diffuse_bind_group2: wgpu::BindGroup,
    x_ctr: f32,
    y_ctr: f32,
    shape: Shape,
    i: usize,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bytes = include_bytes!("sn2_cd1_rnl_gray.tif");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "sn2_cd1_rnl_gray.tif")
                .unwrap();

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let diffuse_bytes = include_bytes!("sn10_cd1_rnl_gray.tif");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "sn10_cd1_rnl_gray.tif")
                .unwrap();

        let diffuse_bind_group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let diffuse_bytes = include_bytes!("sn15_cd3_campus_gray.tif");
        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "sn15_cd3_campus_gray.tif",
        )
        .unwrap();

        let diffuse_bind_group2 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let shader_with_texture = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("TextureShader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_with_texture.wgsl").into()),
        });

        let pipeline_with_texture_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline_with_texture =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Texture Render Pipeline"),
                layout: Some(&pipeline_with_texture_layout),
                vertex: wgpu::VertexState {
                    module: &shader_with_texture,
                    entry_point: "vs_main",
                    buffers: &[VertexTexture::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_with_texture,
                    entry_point: "fs_main",
                    targets: &[wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
            });
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let x_ctr = 0.7f32;
        let y_ctr = 0.7f32;
        let bundle = create_circle_bundle(
            &device,
            &config,
            &shader,
            &pipeline_layout,
            1u32,
            x_ctr,
            y_ctr,
        );
        let rebuild_bundle = false;
        let shape = Shape::Circle;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shader,
            pipeline_layout,
            shader_with_texture,
            pipeline_with_texture_layout,
            pipeline_with_texture,
            vertex_buffer,
            index_buffer,
            num_indices,
            bundle,
            rebuild_bundle,
            diffuse_texture,
            diffuse_bind_group,
            diffuse_bind_group1,
            diffuse_bind_group2,
            x_ctr,
            y_ctr,
            shape,
            i: 0usize,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let ElementState::Pressed = input.state {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Left) => {
                            self.x_ctr += 0.05;
                            self.rebuild_bundle = true;
                        }
                        Some(VirtualKeyCode::Right) => {
                            self.x_ctr -= 0.05;
                            self.rebuild_bundle = true;
                        }
                        Some(VirtualKeyCode::Up) => {
                            self.y_ctr += 0.05;
                            self.rebuild_bundle = true;
                        }
                        Some(VirtualKeyCode::Down) => {
                            self.y_ctr -= 0.05;
                            self.rebuild_bundle = true;
                        }
                        _ => match self.shape {
                            Shape::Circle => {
                                self.shape = Shape::Square;
                                self.rebuild_bundle = true;
                            }
                            Shape::Square => {
                                self.shape = Shape::Cross;
                                self.rebuild_bundle = true;
                            }
                            Shape::Cross => {
                                self.shape = Shape::Circle;
                                self.rebuild_bundle = true;
                            }
                        },
                    }
                }
            }
            _ => {}
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if self.rebuild_bundle {
            self.bundle = match self.shape {
                Shape::Circle => create_circle_bundle(
                    &self.device,
                    &self.config,
                    &self.shader,
                    &self.pipeline_layout,
                    1u32,
                    self.x_ctr,
                    self.y_ctr,
                ),
                Shape::Square => create_square_bundle(
                    &self.device,
                    &self.config,
                    &self.shader,
                    &self.pipeline_layout,
                    1u32,
                    self.x_ctr,
                    self.y_ctr,
                ),
                Shape::Cross => create_cross_bundle(
                    &self.device,
                    &self.config,
                    &self.shader,
                    &self.pipeline_layout,
                    1u32,
                    self.x_ctr,
                    self.y_ctr,
                ),
            };
            self.rebuild_bundle = false;
        }
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.execute_bundles(iter::once(&self.bundle));

            render_pass.set_pipeline(&self.pipeline_with_texture);
            {
                if self.i == 0 {
                    render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                    self.i = 1;
                } else if self.i == 1 {
                    render_pass.set_bind_group(0, &self.diffuse_bind_group1, &[]);
                    self.i = 2;
                } else if self.i == 2 {
                    render_pass.set_bind_group(0, &self.diffuse_bind_group2, &[]);
                    self.i = 0;
                }
            }
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = pollster::block_on(State::new(&window));

    let mut last_frame_inst = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => state.update(event),
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
                if last_frame_inst.elapsed().as_millis() > 17 {
                    println!("Frame was skipped {:?}", last_frame_inst.elapsed());
                }
                last_frame_inst = Instant::now();
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

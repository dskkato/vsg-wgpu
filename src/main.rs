use std::iter;

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

enum Shape {
    Square,
    Circle,
    Cross,
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    shader: wgpu::ShaderModule,
    pipeline_layout: wgpu::PipelineLayout,
    size: winit::dpi::PhysicalSize<u32>,
    bundle: wgpu::RenderBundle,
    rebuild_circle_bundle: bool,
    x_ctr: f32,
    y_ctr: f32,
    shape: Shape,
}

fn create_cross_bundle(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    pipeline_layout: &wgpu::PipelineLayout,
    sample_count: u32,
    x_ctr: f32,
    y_ctr: f32,
) -> wgpu::RenderBundle {
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
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

    let x_ctr = x_ctr.abs();
    let y_ctr = y_ctr.abs();
    let vertices = [
        Vertex {
            position: [-x_ctr / 2.0, -y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [-x_ctr / 2.0, y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [x_ctr / 2.0, -y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [x_ctr / 2.0, y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [-x_ctr, -y_ctr / 2.0, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [-x_ctr, y_ctr / 2.0, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [x_ctr, -y_ctr / 2.0, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [x_ctr, y_ctr / 2.0, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
    ];

    let indices = [0u16, 2, 1, 2, 3, 1, 4, 6, 5, 6, 7, 5];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = indices.len() as u32;

    let mut encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: Some("Render Bundle Encoder"),
        color_formats: &[config.format],
        depth_stencil: None,
        sample_count,
        multiview: None,
    });
    encoder.set_pipeline(&render_pipeline);
    encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    encoder.draw_indexed(0..num_indices, 0, 0..1);
    encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("Square"),
    })
}

fn create_square_bundle(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    pipeline_layout: &wgpu::PipelineLayout,
    sample_count: u32,
    x_ctr: f32,
    y_ctr: f32,
) -> wgpu::RenderBundle {
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
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

    let x_ctr = x_ctr.abs();
    let y_ctr = y_ctr.abs();
    let vertices = [
        Vertex {
            position: [-x_ctr, -y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [-x_ctr, y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [x_ctr, -y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [x_ctr, y_ctr, 0.0f32],
            color: [1.0, 1.0, 1.0],
        },
    ];

    let indices = [0u16, 2, 1, 2, 3, 1];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = indices.len() as u32;

    let mut encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: Some("Render Bundle Encoder"),
        color_formats: &[config.format],
        depth_stencil: None,
        sample_count,
        multiview: None,
    });
    encoder.set_pipeline(&render_pipeline);
    encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    encoder.draw_indexed(0..num_indices, 0, 0..1);
    encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("Square"),
    })
}

fn create_circle_bundle(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    pipeline_layout: &wgpu::PipelineLayout,
    sample_count: u32,
    x_ctr: f32,
    y_ctr: f32,
) -> wgpu::RenderBundle {
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
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

    let n_vertices = 1024;
    let mut vertices = Vec::new();
    let v = Vertex {
        position: [-x_ctr, y_ctr, 0.0f32],
        color: [1.0, 1.0, 1.0],
    };
    vertices.push(v);
    for i in 0..n_vertices {
        let v = Vertex {
            position: [
                0.5f32 * (6.28 as f32 * i as f32 / n_vertices as f32).cos() - x_ctr,
                0.5f32 * (6.28 as f32 * i as f32 / n_vertices as f32).sin() + y_ctr,
                0.0f32,
            ],
            color: [0.5, 0.2, 0.1],
        };
        vertices.push(v);
    }

    let mut indices = Vec::new();
    for i in 0..n_vertices - 1 {
        indices.push(0u16);
        indices.push(i as u16);
        indices.push(i as u16 + 1);
    }
    indices.push(0u16);
    indices.push((n_vertices - 1) as u16);
    indices.push(1u16);

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = indices.len() as u32;

    let mut encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: Some("Render Bundle Encoder"),
        color_formats: &[config.format],
        depth_stencil: None,
        sample_count,
        multiview: None,
    });
    encoder.set_pipeline(&render_pipeline);
    encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    encoder.draw_indexed(0..num_indices, 0, 0..1);
    encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("Circle"),
    })
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

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let x_ctr = 0.0f32;
        let y_ctr = 0.0f32;
        let circle_bundle = create_circle_bundle(
            &device,
            &config,
            &shader,
            &pipeline_layout,
            1u32,
            x_ctr,
            y_ctr,
        );
        let rebuild_circle_bundle = false;
        let shape = Shape::Circle;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shader,
            pipeline_layout,
            bundle: circle_bundle,
            rebuild_circle_bundle,
            x_ctr,
            y_ctr,
            shape,
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
                            self.rebuild_circle_bundle = true;
                        }
                        Some(VirtualKeyCode::Right) => {
                            self.x_ctr -= 0.05;
                            self.rebuild_circle_bundle = true;
                        }
                        Some(VirtualKeyCode::Up) => {
                            self.y_ctr += 0.05;
                            self.rebuild_circle_bundle = true;
                        }
                        Some(VirtualKeyCode::Down) => {
                            self.y_ctr -= 0.05;
                            self.rebuild_circle_bundle = true;
                        }
                        _ => match self.shape {
                            Shape::Circle => {
                                self.shape = Shape::Square;
                                self.rebuild_circle_bundle = true;
                            }
                            Shape::Square => {
                                self.shape = Shape::Cross;
                                self.rebuild_circle_bundle = true;
                            }
                            Shape::Cross => {
                                self.shape = Shape::Circle;
                                self.rebuild_circle_bundle = true;
                            }
                        },
                    }
                }
            }
            _ => {}
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if self.rebuild_circle_bundle {
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
            self.rebuild_circle_bundle = false;
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

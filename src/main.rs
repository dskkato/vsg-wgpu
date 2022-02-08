use std::io::Read;
use std::net::TcpListener;
use std::time::{Duration, Instant};
use std::{iter, thread};

use log::info;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod texture;
mod vertex;
use vertex::VertexTexture;
mod renderers;
use renderers::*;

mod messages;
use messages::Coordinates;
use messages::Shape;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    picture: Picture,
    size: winit::dpi::PhysicalSize<u32>,
    bundle: Box<dyn renderers::StimulusRenderer>,
    rebuild_bundle: bool,
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

        let x_ctr = 0.7f32;
        let y_ctr = 0.7f32;
        let bundle = Box::new(Circle::new(&device, &config.format, x_ctr, y_ctr));
        let rebuild_bundle = false;
        let shape = Shape::Circle {
            radius: 0.2f32,
            ctr: Coordinates { x: x_ctr, y: y_ctr },
        };

        let picture = Picture::new(&device, &queue, &config.format, x_ctr, y_ctr);

        Self {
            surface,
            device,
            queue,
            config,
            picture,
            size,
            bundle,
            rebuild_bundle,
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
                            Shape::Circle { .. } => {
                                self.shape = Shape::Square {
                                    size: 0.3f32,
                                    ctr: Coordinates {
                                        x: self.x_ctr,
                                        y: self.y_ctr,
                                    },
                                };
                                self.rebuild_bundle = true;
                            }
                            Shape::Square { .. } => {
                                self.shape = Shape::Cross {
                                    size: 0.3f32,
                                    line_width: 0.132,
                                    ctr: Coordinates {
                                        x: self.x_ctr,
                                        y: self.y_ctr,
                                    },
                                };
                                self.rebuild_bundle = true;
                            }
                            Shape::Cross { .. } => {
                                self.shape = Shape::Circle {
                                    radius: 0.2f32,
                                    ctr: Coordinates {
                                        x: self.x_ctr,
                                        y: self.y_ctr,
                                    },
                                };
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
                Shape::Circle { .. } => Box::new(Circle::new(
                    &self.device,
                    &self.config.format,
                    self.x_ctr,
                    self.y_ctr,
                )),
                Shape::Square { .. } => Box::new(Rectangle::new(
                    &self.device,
                    &self.config.format,
                    self.x_ctr,
                    self.y_ctr,
                )),
                Shape::Cross { .. } => Box::new(Cross::new(
                    &self.device,
                    &self.config.format,
                    self.x_ctr,
                    self.y_ctr,
                )),
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
            self.bundle.render(&mut render_pass);
            self.picture.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::<&str>::with_user_event();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = pollster::block_on(State::new(&window));

    let mut last_frame_inst = Instant::now();

    let event_loop_proxy = event_loop.create_proxy();
    let handler = thread::spawn(move || {
        let one_second = Duration::from_millis(1000);
        let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
        loop {
            for stream in listner.incoming() {
                let mut stream = stream.unwrap();
                println!("Connection established! : {}", stream.peer_addr().unwrap());

                // handle_connection
                let msg: serde_json::Result<messages::Message> = serde_json::from_reader(stream);
                if msg.is_err() {
                    println!("Error: {}", msg.err().unwrap());
                    continue;
                }

                println!("Contents : {:?}", msg);
                event_loop_proxy
                    .send_event("Start")
                    .expect("Failed to send event");
            }
        }
    });
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // info!("{:?}", event);
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
                    // println!("Frame was skipped {:?}", last_frame_inst.elapsed());
                }
                last_frame_inst = Instant::now();
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            Event::UserEvent(event) => {
                println!("UserEvent : {}", event);
            }
            _ => {}
        }
    });
}

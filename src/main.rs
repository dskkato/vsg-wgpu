use std::io::Write;
use std::thread;
use std::time::Instant;
use std::{io::Read, net::TcpListener};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod graphics;
mod renderers;
mod texture;
mod vertex;
use renderers::*;

mod messages;
use messages::Shape;
use messages::{Command, Coordinates};

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
    bg_color: wgpu::Color,
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
        let bundle = Box::new(Circle::new(
            &device,
            &config.format,
            x_ctr,
            y_ctr,
            0.2,
            &[0.2, 0.0, 0.0, 1.0],
        ));
        let rebuild_bundle = false;
        let shape = Shape::Circle {
            radius: 0.2f32,
            ctr: Coordinates { x: x_ctr, y: y_ctr },
        };

        let picture = Picture::new(&device, &queue, &config.format);
        let bg_color = wgpu::Color::BLACK;

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
            bg_color,
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
                            self.x_ctr -= 0.05;
                            self.rebuild_bundle = true;
                        }
                        Some(VirtualKeyCode::Right) => {
                            self.x_ctr += 0.05;
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

    pub fn update_shape(&mut self, shape: Shape) {
        self.bundle = match shape {
            Shape::Circle { radius, ctr } => Box::new(Circle::new(
                &self.device,
                &self.config.format,
                ctr.x,
                ctr.y,
                radius,
                &[0.2, 0.0, 0.0, 1.0],
            )),
            Shape::Square { size, ctr } => Box::new(Rectangle::new(
                &self.device,
                &self.config.format,
                ctr.x,
                ctr.y,
                size,
                size,
                &[0.0, 0.2, 0.0, 1.0],
            )),
            Shape::Cross {
                size,
                line_width,
                ctr,
            } => Box::new(Cross::new(
                &self.device,
                &self.config.format,
                ctr.x,
                ctr.y,
                size,
                size,
                line_width,
                &[0.0, 0.0, 0.2, 1.0],
            )),
        };
    }

    pub fn update_bg_color(&mut self, bg_color: &[f64; 4]) {
        self.bg_color = wgpu::Color {
            r: bg_color[0],
            g: bg_color[1],
            b: bg_color[2],
            a: bg_color[3],
        };
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if self.rebuild_bundle {
            self.bundle = match self.shape {
                Shape::Circle { .. } => Box::new(Circle::new(
                    &self.device,
                    &self.config.format,
                    self.x_ctr,
                    self.y_ctr,
                    0.2,
                    &[0.2, 0.0, 0.0, 1.0],
                )),
                Shape::Square { .. } => Box::new(Rectangle::new(
                    &self.device,
                    &self.config.format,
                    self.x_ctr,
                    self.y_ctr,
                    0.9,
                    0.7,
                    &[0.0, 0.2, 0.0, 1.0],
                )),
                Shape::Cross { .. } => Box::new(Cross::new(
                    &self.device,
                    &self.config.format,
                    self.x_ctr,
                    self.y_ctr,
                    0.9,
                    0.6,
                    0.1,
                    &[0.0, 0.0, 0.2, 1.0],
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
                        load: wgpu::LoadOp::Clear(self.bg_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            self.picture.render(&mut render_pass);
            self.bundle.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::<Command>::with_user_event();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = pollster::block_on(State::new(&window));

    let mut last_frame_inst = Instant::now();

    let event_loop_proxy = event_loop.create_proxy();
    let _handler = thread::spawn(move || {
        let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
        let mut buffer = [0; 1024];
        loop {
            for stream in listner.incoming() {
                let mut stream = stream.unwrap();
                println!("Connection established! : {}", stream.peer_addr().unwrap());
                loop {
                    let msg: serde_json::Result<messages::Message> = match stream.read(&mut buffer)
                    {
                        Ok(n) => {
                            if n == 0 {
                                println!("Connection closed!");
                                stream.write(b"{\"type\": \"close\"}").unwrap();
                                stream.flush().unwrap();
                                break;
                            }
                            serde_json::from_slice(&buffer[0..n])
                        }
                        _ => {
                            stream.write(b"{\"type\": \"err\"}").unwrap();
                            stream.flush().unwrap();
                            continue;
                        }
                    };

                    // handle_connection
                    if msg.is_err() {
                        println!("Error: {}", msg.err().unwrap());
                        stream.write(b"{\"type\": \"err\"}").unwrap();
                        stream.flush().unwrap();
                        continue;
                    }

                    println!("Contents : {:?}", msg);
                    match msg.unwrap() {
                        messages::Message::SetShape(shape) => {
                            event_loop_proxy
                                .send_event(Command::Draw(shape))
                                .expect("Failed to send event");
                        }
                        messages::Message::SetBgColor(color) => {
                            event_loop_proxy
                                .send_event(Command::Clear(color))
                                .expect("Failed to send event");
                        }
                    }
                    stream.write(b"{\"type\": \"success\"}").unwrap();
                    stream.flush().unwrap();
                }
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
                println!("UserEvent : {:?}", event);
                match event {
                    Command::Draw(shape) => {
                        state.update_shape(shape);
                    }
                    Command::Clear(color) => {
                        state.update_bg_color(&color);
                    }
                }
            }
            _ => {}
        }
    });
}

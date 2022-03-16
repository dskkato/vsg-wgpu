use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::{io::Read, net::TcpListener};

use env_logger::TimestampPrecision;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    monitor::MonitorHandle,
    window::{Fullscreen, Window, WindowBuilder},
};

use anyhow::Result;
use clap::Parser;

mod graphics;
mod renderers;
mod texture;
mod vertex;
use renderers::scene::Scene;
use renderers::*;

mod messages;
use messages::{Command, Coordinates};

use prost::Message;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value = "7878")]
    port: String,

    #[clap(short, long)]
    fullscreen: bool,

    #[clap(short, long, default_value = "0")]
    monitor: usize,
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    picture: Option<Picture>,
    size: winit::dpi::PhysicalSize<u32>,
    scene: Scene,
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
                    limits: wgpu::Limits::downlevel_defaults(),
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

        let bg_color = wgpu::Color::BLACK;

        let scene = renderers::scene::Scene::new();

        Self {
            surface,
            device,
            queue,
            config,
            picture: None,
            size,
            scene,
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

    pub fn update_shape(&mut self, shape: &vsg_messages::shape::Shape) {
        use vsg_messages::shape::Shape;
        let bundle: Box<dyn StimulusRenderer> = match shape {
            Shape::Circle(vsg_messages::Circle { radius, ctr }) => {
                let ctr = ctr.as_ref().unwrap();
                Box::new(Circle::new(
                    &self.device,
                    &self.config.format,
                    ctr.x,
                    ctr.y,
                    *radius,
                    &[0.2, 0.0, 0.0, 1.0],
                ))
            }
            Shape::Square(vsg_messages::Square { size, ctr }) => {
                let ctr = ctr.as_ref().unwrap();
                Box::new(Rectangle::new(
                    &self.device,
                    &self.config.format,
                    ctr.x,
                    ctr.y,
                    *size,
                    *size,
                    &[0.0, 0.2, 0.0, 1.0],
                ))
            }
            Shape::Cross(vsg_messages::Cross {
                size,
                line_width,
                ctr,
            }) => {
                let ctr = ctr.as_ref().unwrap();
                Box::new(Cross::new(
                    &self.device,
                    &self.config.format,
                    ctr.x,
                    ctr.y,
                    *size,
                    *size,
                    *line_width,
                    &[0.0, 0.0, 0.2, 1.0],
                ))
            }
        };
        let mut scene = Scene::new();
        scene.add_stimulus(bundle);
        self.scene = scene;
    }

    pub fn update_bg_color(&mut self, bg_color: &[f32; 4]) {
        self.bg_color = wgpu::Color {
            r: bg_color[0] as f64,
            g: bg_color[1] as f64,
            b: bg_color[2] as f64,
            a: bg_color[3] as f64,
        };
    }

    pub fn update_texture(&mut self, _idx: u32, texture: &[u8]) {
        self.picture = Some(Picture::new(
            &self.device,
            &self.queue,
            &self.config.format,
            texture,
        ));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            if let Some(image) = &self.picture {
                image.render(&mut render_pass);
            }
            self.scene.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn handle_connection(
    mut stream: TcpStream,
    message_bucket: Arc<Mutex<Vec<Command>>>,
) -> Result<()> {
    let mut msg_size = [0; 4];
    let mut buffer = vec![0; 1024];

    log::debug!("Connection established! : {}", stream.peer_addr()?);
    loop {
        stream.read_exact(&mut msg_size)?;
        let len = u32::from_be_bytes(msg_size) as usize;
        log::trace!("{}", len);
        if len > buffer.len() {
            buffer.resize(len, 0);
        }
        stream.read_exact(&mut buffer[..len])?;
        let msg = vsg_messages::RootMessage::decode(&buffer[..len])?;

        log::debug!("Contents : {:?}", msg);
        match msg.command {
            Some(vsg_messages::root_message::Command::SetShape(shape)) => {
                let mut t = message_bucket.lock().unwrap();
                t.push(Command::Draw(shape.shape.unwrap()));
            }
            Some(vsg_messages::root_message::Command::SetBgColor(color)) => {
                let mut t = message_bucket.lock().unwrap();
                let color = [
                    color.color[0],
                    color.color[1],
                    color.color[2],
                    color.color[3],
                ];
                t.push(Command::Clear(color));
            }
            Some(vsg_messages::root_message::Command::SetTexture(texture)) => {
                let idx = texture.index;
                let data = texture.data;
                let mut t = message_bucket.lock().unwrap();
                t.push(Command::Texture(idx, data));
            }
            None => {
                log::error!("Unknown command");
                break;
            }
        }
        let msg = b"{\"type\": \"success\"}";
        let len = msg.len() as u32;
        log::trace!("{}", len);
        stream.write(&len.to_be_bytes())?;
        stream.write(msg)?;
        stream.flush()?;
    }
    Ok(())
}

// Enumerate monitors and prompt user to choose one
fn prompt_for_monitor(event_loop: &EventLoop<()>, idx: usize) -> MonitorHandle {
    for (num, monitor) in event_loop.available_monitors().enumerate() {
        log::debug!("Monitor #{}: {:?}", num, monitor.name());
    }

    let monitor = event_loop
        .available_monitors()
        .nth(idx)
        .expect("Please enter a valid ID");

    monitor
}

fn main() {
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Millis))
        .init();
    let args = Args::parse();
    log::debug!("{:?}", &args);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(false)
        // .with_inner_size(PhysicalSize::new(1920u32, 1080u32))
        .build(&event_loop)
        .unwrap();

    if args.fullscreen {
        // window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        let monitor = prompt_for_monitor(&event_loop, args.monitor);
        window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
    }
    window.set_cursor_visible(false);
    match window.set_cursor_grab(true) {
        Ok(()) => log::debug!("Grabbing cursor"),
        Err(e) => log::error!("{:?}", e),
    };

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = pollster::block_on(State::new(&window));
    window.set_visible(true);

    let mut last_frame_inst = Instant::now();

    let message_bucket = Arc::new(Mutex::new(Vec::<Command>::new()));
    let send_message = Arc::clone(&message_bucket);
    let _handler = thread::spawn(move || {
        let host = &args.host;
        let port = &args.port;
        let listner = TcpListener::bind(format!("{host}:{port}")).unwrap();
        loop {
            for stream in listner.incoming() {
                let stream = stream.unwrap();
                match handle_connection(stream, Arc::clone(&send_message)) {
                    Ok(_) => (),
                    Err(e) => log::error!("{}", e),
                }
            }
        }
    });
    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Wait;
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
                    _ => {} //state.update(event),
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // window.request_redraw(); is not called and this pass will not be executed.
                // This is for continuous rendering.
            }
            Event::MainEventsCleared => {
                log::trace!("MainEventsCleared");
                {
                    let mut t = message_bucket.lock().unwrap();
                    // 暫定：最後のコマンドだけを実行する
                    match t.last() {
                        Some(Command::Draw(shape)) => state.update_shape(&shape),
                        Some(Command::Clear(color)) => state.update_bg_color(&color),
                        Some(Command::Texture(idx, texture)) => state.update_texture(*idx, texture),
                        _ => {}
                    }
                    t.clear();
                }
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => log::error!("{:?}", e),
                }
                log::trace!("Render finished");
                if last_frame_inst.elapsed().as_millis() > 20 {
                    log::info!("Frame was skipped {:?}", last_frame_inst.elapsed());
                }
                last_frame_inst = Instant::now();
            }
            _ => {}
        }
    });
}

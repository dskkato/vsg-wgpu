mod circle;
pub use circle::Circle;
mod cross;
pub use cross::Cross;
mod picture;
pub use picture::Picture;
mod rectangle;
pub use rectangle::Rectangle;

pub trait StimulusRenderer {
    // fn new(device: &wgpu::Device, format:&wgpu::TextureFormat) -> Self;

    // Prepare for rendering, create all resources used during render, storing render data internally
    fn prepare(&mut self);

    // Render using internal data and user provided renderpass
    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass;
}

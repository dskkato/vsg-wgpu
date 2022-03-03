mod circle;
pub use circle::Circle;
mod cross;
pub use cross::Cross;
mod image;
pub use image::Picture;
mod rectangle;
pub use rectangle::Rectangle;
pub mod triangle;

pub mod scene;

pub trait StimulusRenderer {
    // Render using internal data and user provided renderpass
    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass;
}

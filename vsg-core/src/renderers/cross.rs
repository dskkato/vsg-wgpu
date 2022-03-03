use super::{Rectangle, StimulusRenderer};
use crate::Coordinates;

pub struct Cross {
    pub ctr: Coordinates,
    pipelines: [Rectangle; 2],
}

impl Cross {
    pub fn new(
        device: &wgpu::Device,
        format: &wgpu::TextureFormat,
        x_ctr: f32,
        y_ctr: f32,
        x_size: f32,
        y_size: f32,
        line_width: f32,
        color: &[f32; 4],
    ) -> Self {
        // horizontal line
        let p1 = Rectangle::new(device, format, x_ctr, y_ctr, x_size, line_width, color);
        let p2 = Rectangle::new(device, format, x_ctr, y_ctr, line_width, y_size, color);

        Self {
            ctr: Coordinates { x: x_ctr, y: y_ctr },
            pipelines: [p1, p2],
        }
    }
}

impl StimulusRenderer for Cross {
    // Render using internal data and user provided renderpass
    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass,
    {
        for p in &self.pipelines {
            p.render(render_pass);
        }
    }
}

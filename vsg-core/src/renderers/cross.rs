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
        ctr: &Coordinates,
        size: &Coordinates,
        line_width: f32,
        color: &[f32; 4],
    ) -> Self {
        // horizontal line
        let p1 = Rectangle::new(device, format, ctr.x, ctr.y, size.x, line_width, color);
        let p2 = Rectangle::new(device, format, ctr.x, ctr.y, line_width, size.y, color);

        Self {
            ctr: ctr.clone(),
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

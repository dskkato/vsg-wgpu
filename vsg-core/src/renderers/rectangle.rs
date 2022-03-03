use super::{
    triangle::{self, Triangle},
    StimulusRenderer,
};
use crate::graphics::triangle::Vertex2D;
use crate::Coordinates;

pub struct Rectangle {
    pub size: f32,
    pub ctr: Coordinates,
    pipeline: triangle::Triangle,
}

impl Rectangle {
    pub fn new(
        device: &wgpu::Device,
        format: &wgpu::TextureFormat,
        x_ctr: f32,
        y_ctr: f32,
        width: f32,
        height: f32,
        color: &[f32; 4],
    ) -> Self {
        let vertices = [
            Vertex2D {
                position: [x_ctr - width / 2.0, y_ctr - height / 2.0],
                color: *color,
            },
            Vertex2D {
                position: [x_ctr - width / 2.0, y_ctr + height / 2.0],
                color: *color,
            },
            Vertex2D {
                position: [x_ctr + width / 2.0, y_ctr - height / 2.0],
                color: *color,
            },
            Vertex2D {
                position: [x_ctr + width / 2.0, y_ctr + height / 2.0],
                color: *color,
            },
        ];

        let indices = [0u16, 2, 1, 2, 3, 1];

        let mut pipeline = Triangle::new(device, format);
        pipeline.prepare(device, &vertices, &indices);

        Self {
            size: 0.3,
            ctr: Coordinates { x: x_ctr, y: y_ctr },
            pipeline,
        }
    }
}

impl StimulusRenderer for Rectangle {
    // Render using internal data and user provided renderpass
    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass,
    {
        self.pipeline.render(render_pass)
    }
}

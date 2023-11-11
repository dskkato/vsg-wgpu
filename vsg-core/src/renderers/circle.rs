use super::{triangle::Triangle, StimulusRenderer};
use crate::{graphics::triangle::Vertex2D, Coordinates};

pub struct Circle {
    pub size: f32,
    pub ctr: Coordinates,
    triangles: Triangle,
}

impl Circle {
    pub fn new(
        device: &wgpu::Device,
        format: &wgpu::TextureFormat,
        x_ctr: f32,
        y_ctr: f32,
        radius: f32,
        color: &[f32; 4],
    ) -> Self {
        let n_vertices = 1024;
        let mut vertices = Vec::new();
        let v = Vertex2D {
            position: [x_ctr, y_ctr],
            color: *color,
        };
        vertices.push(v);
        for i in 0..n_vertices {
            let v = Vertex2D {
                position: [
                    radius * (std::f32::consts::TAU * i as f32 / n_vertices as f32).cos() + x_ctr,
                    radius * (std::f32::consts::TAU * i as f32 / n_vertices as f32).sin() + y_ctr,
                ],
                color: *color,
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

        let mut triangles = Triangle::new(device, format);
        triangles.prepare(device, &vertices, &indices);

        Self {
            size: 0.3,
            ctr: Coordinates { x: x_ctr, y: y_ctr },
            triangles,
        }
    }
}

impl StimulusRenderer for Circle {
    // Render using internal data and user provided renderpass
    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass,
    {
        self.triangles.render(render_pass)
    }
}

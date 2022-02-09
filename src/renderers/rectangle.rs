use std::iter;

use super::StimulusRenderer;
use crate::{vertex::Vertex, Coordinates};
use wgpu::util::DeviceExt;

pub struct Rectangle {
    pub size: f32,
    pub ctr: Coordinates,
    bundle: wgpu::RenderBundle,
}

impl Rectangle {
    pub fn new(
        device: &wgpu::Device,
        format: &wgpu::TextureFormat,
        x_ctr: f32,
        y_ctr: f32,
    ) -> Self {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: *format,
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
        let sample_count = 1;
        let mut encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("Render Bundle Encoder"),
                color_formats: &[*format],
                depth_stencil: None,
                sample_count,
                multiview: None,
            });
        encoder.set_pipeline(&render_pipeline);
        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        encoder.draw_indexed(0..num_indices, 0, 0..1);
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("Square"),
        });

        Self {
            size: 0.3,
            ctr: Coordinates { x: x_ctr, y: y_ctr },
            bundle,
        }
    }
}

impl StimulusRenderer for Rectangle {
    // Prepare for rendering, create all resources used during render, storing render data internally
    fn prepare(&mut self) {}

    // Render using internal data and user provided renderpass
    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass,
    {
        render_pass.execute_bundles(iter::once(&self.bundle));
    }
}
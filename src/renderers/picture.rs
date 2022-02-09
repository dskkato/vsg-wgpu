use super::StimulusRenderer;
use crate::{texture, vertex::VertexTexture};
use wgpu::util::DeviceExt;

const VERTICES: &[VertexTexture] = &[
    VertexTexture {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    }, // upper right
    VertexTexture {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 0.0],
    }, // upper left
    VertexTexture {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    }, // lower right
    VertexTexture {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    }, // lower left
];

const INDICES: &[u16] = &[0, 1, 2, 1, 3, 2];

pub struct Picture {
    pipeline_with_texture: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    diffuse_bind_group1: wgpu::BindGroup,
    #[allow(dead_code)]
    diffuse_bind_group2: wgpu::BindGroup,
}

impl Picture {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: &wgpu::TextureFormat) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bytes = include_bytes!("sn2_cd1_rnl_gray.tif");
        let diffuse_texture = crate::texture::Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "sn2_cd1_rnl_gray.tif",
        )
        .unwrap();

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let diffuse_bytes = include_bytes!("sn10_cd1_rnl_gray.tif");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "sn10_cd1_rnl_gray.tif")
                .unwrap();

        let diffuse_bind_group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let diffuse_bytes = include_bytes!("sn15_cd3_campus_gray.tif");
        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "sn15_cd3_campus_gray.tif",
        )
        .unwrap();

        let diffuse_bind_group2 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let shader_with_texture = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("TextureShader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../shaders/shader_with_texture.wgsl").into(),
            ),
        });

        let pipeline_with_texture_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline_with_texture =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Texture Render Pipeline"),
                layout: Some(&pipeline_with_texture_layout),
                vertex: wgpu::VertexState {
                    module: &shader_with_texture,
                    entry_point: "vs_main",
                    buffers: &[VertexTexture::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_with_texture,
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
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        Self {
            pipeline_with_texture,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_bind_group1,
            diffuse_bind_group2,
        }
    }
}

impl StimulusRenderer for Picture {
    fn prepare(&mut self) {}

    fn render<'rpass, 'pass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'pass>)
    where
        'rpass: 'pass,
    {
        render_pass.set_pipeline(&self.pipeline_with_texture);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        // {
        //     match self.i / 60 {
        //         0 => render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]),
        //         1 => render_pass.set_bind_group(0, &self.diffuse_bind_group1, &[]),
        //         2 => render_pass.set_bind_group(0, &self.diffuse_bind_group2, &[]),
        //         _ => {
        //             render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        //             self.i = 1;
        //         }
        //     }
        //     self.i += 1;
        // }
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

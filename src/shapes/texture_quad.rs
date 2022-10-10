use std::fs::File;
use std::io::{BufReader, Read};
use wgpu::{BindGroup, BindGroupLayoutEntry, Buffer, BufferAddress, Device, IndexFormat, Queue, RenderPass, RenderPipeline, VertexBufferLayout};
use wgpu::util::DeviceExt;
use crate::shape::Vertex;
use crate::{Shape, State};
use super::super::texture::Texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    pub position: [f32; 3],
    pub texture_cords: [f32; 2],
}

impl Vertex for TextureVertex {
    fn get_descriptor<'a>() -> VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ]
        }
    }
}

pub struct TextureQuad {
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),

    texture_top_left: (f32,f32),
    texture_bottom_right: (f32,f32),

    vertex_buffer: Buffer,
    indices_buffer: Buffer,

    render_pipeline: RenderPipeline,

    diffuse_bind_group: BindGroup,

}

impl TextureQuad {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32), png: String, texture_cord_top_left: (f32, f32), texture_cord_bottom_right: (f32, f32), device: &Device, queue: &Queue, state: &State) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("../textured.wgsl").into()),
        });

        let f = File::open(png).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer);

        let diffuse_bytes = buffer.as_slice(); // CHANGED!
        let diffuse_texture = Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

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
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view), // CHANGED!
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler), // CHANGED!
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    TextureVertex::get_descriptor(),
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: state.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        Self {

            top_left,
            bottom_right,
            texture_top_left: texture_cord_top_left,
            texture_bottom_right: texture_cord_bottom_right,
            vertex_buffer: Self::generate_vertex_buffer(&top_left, &bottom_right, &texture_cord_top_left, &texture_cord_bottom_right, &device),
            indices_buffer: Self::generate_indices_buffer(&device),
            render_pipeline,
            diffuse_bind_group
        }
    }

    fn generate_vertex_buffer(top_left: &(f32, f32), bottom_right: &(f32, f32), texture_cord_top_left: &(f32, f32), texture_cord_bottom_right: &(f32, f32), device: &Device) -> Buffer {
        let vertices: &[TextureVertex] = &[
            TextureVertex { position: [top_left.0, top_left.1, 0.0], texture_cords: [texture_cord_top_left.0, texture_cord_top_left.1] },
            TextureVertex { position: [top_left.0, bottom_right.1, 0.0], texture_cords: [texture_cord_top_left.0, texture_cord_bottom_right.1] },
            TextureVertex { position: [bottom_right.0, bottom_right.1, 0.0], texture_cords: [texture_cord_bottom_right.0, texture_cord_bottom_right.1] },
            TextureVertex { position: [bottom_right.0, top_left.1, 0.0], texture_cords: [texture_cord_bottom_right.0, texture_cord_top_left.1] },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        vertex_buffer
    }

    fn generate_indices_buffer(device: &Device) -> Buffer {
        let indices: &[u16] = &[
            0, 1, 2,
            0, 2, 3
        ];

        let indices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        indices_buffer
    }
}

impl Shape for TextureQuad {
    fn get_vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    fn update_vertex_buffer(&mut self, device: &Device) {
        self.vertex_buffer = TextureQuad::generate_vertex_buffer(&self.top_left, &self.bottom_right, &self.texture_top_left, &self.texture_bottom_right, &device);
    }

    fn get_indices_buffer(&self) -> &Buffer {
        &self.indices_buffer
    }

    fn update_indices_buffer(&mut self, device: &Device) {
        self.indices_buffer = TextureQuad::generate_indices_buffer(&device);
    }

    fn get_number_indices(&self) -> u32 {
        6
    }

    fn get_render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }

    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(self.get_render_pipeline());

        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.get_vertex_buffer().slice(..));
        render_pass.set_index_buffer(self.get_indices_buffer().slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.get_number_indices(), 0, 0..1);
    }
}


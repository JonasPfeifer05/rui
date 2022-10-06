use image::flat::View;
use wgpu::{Buffer, BufferAddress, CommandEncoder, Device, IndexFormat, RenderPass, RenderPipeline, TextureView, VertexBufferLayout};
use wgpu::util::DeviceExt;
use crate::State;

trait Vertex {
    fn get_descriptor<'a>() -> VertexBufferLayout<'a>;
}

pub trait Shape {
    fn generate_vertex_buffer(&self, device: &Device) -> Buffer;
    fn generate_indices_buffer(&self, device: &Device) -> Buffer;
    fn get_number_indices(&self) -> u32;

    fn generate_render_pipeline(state: &State) -> RenderPipeline;
    //fn generate_render_pass<'a>(state: &State) -> RenderPass<'a>;

    fn draw(&self, state: &State, view: &mut TextureView, encoder: &mut CommandEncoder) {
        let render_pipeline = Self::generate_render_pipeline(state);
        let vertex_buffer = self.generate_vertex_buffer(&state.device);
        let index_buffer = self.generate_indices_buffer(&state.device);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    })
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&render_pipeline);

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.get_number_indices(), 0, 0..1);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for QuadVertex {
    fn get_descriptor<'a>() -> VertexBufferLayout<'a> {
        use std::mem;
        VertexBufferLayout {
            array_stride: mem::size_of::<QuadVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        }
    }
}

pub struct Quadrat {
    pub top_left: (f32,f32),
    pub bottom_right: (f32,f32),
}

impl Quadrat {
    pub fn new(top_left: (f32,f32), bottom_right: (f32,f32)) -> Self {
        Self {
            top_left,
            bottom_right
        }
    }
}

impl Shape for Quadrat {
    fn generate_vertex_buffer(&self, device: &Device) -> Buffer {
        let vertices: &[QuadVertex] = &[
            QuadVertex { position: [self.top_left.0, self.top_left.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [self.top_left.0, self.bottom_right.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [self.bottom_right.0, self.bottom_right.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [self.bottom_right.0, self.top_left.1, 0.0], color: [1.0, 1.0, 0.0] },
        ];

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        })
    }

    fn generate_indices_buffer(&self, device: &Device) -> Buffer {
        let indices: &[u16] = &[
            0, 1, 2,
            0, 2, 3
        ];
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    fn get_number_indices(&self) -> u32 {
        6
    }

    fn generate_render_pipeline(state: &State) -> RenderPipeline {
        let device = &state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("quad.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    QuadVertex::get_descriptor(),
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: state.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
        })
    }
}




use wgpu::{Buffer, BufferAddress, Device, RenderPipeline, VertexBufferLayout};
use wgpu::util::DeviceExt;
use crate::shape::Vertex;
use crate::{Shape, State};

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
            array_stride: mem::size_of::<QuadVertex>() as BufferAddress,
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

pub struct Quad {
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),

    vertex_buffer: Buffer,
    indices_buffer: Buffer,

    render_pipeline: RenderPipeline,
}

impl Quad {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32), device: &Device, state: &State) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("../quad.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        });

        Self {
            top_left,
            bottom_right,
            vertex_buffer: Self::generate_vertex_buffer(&top_left, &bottom_right, &device),
            indices_buffer: Self::generate_indices_buffer(&device),
            render_pipeline,
        }
    }

    fn generate_vertex_buffer(top_left: &(f32, f32), bottom_right: &(f32, f32), device: &Device) -> Buffer {
        let vertices: &[QuadVertex] = &[
            QuadVertex { position: [top_left.0, top_left.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [top_left.0, bottom_right.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [bottom_right.0, bottom_right.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [bottom_right.0, top_left.1, 0.0], color: [1.0, 1.0, 0.0] },
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

impl Shape for Quad {
    fn get_vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    fn update_vertex_buffer(&mut self, device: &Device) {
        self.vertex_buffer = Quad::generate_vertex_buffer(&self.top_left, &self.bottom_right, &device);
    }

    fn get_indices_buffer(&self) -> &Buffer {
        &self.indices_buffer
    }

    fn update_indices_buffer(&mut self, device: &Device) {
        self.indices_buffer = Quad::generate_indices_buffer(&device);
    }

    fn get_number_indices(&self) -> u32 {
        6
    }

    fn get_render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}


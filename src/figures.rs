use std::fmt::Debug;
use wgpu::{Buffer, BufferAddress, BufferUsages, Device, include_wgsl, IndexFormat, RenderPass, RenderPipeline, ShaderModuleDescriptor, ShaderSource, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::State;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl QuadVertex {
    pub fn descriptor<'a>() -> VertexBufferLayout<'a> {
        use std::mem;
        VertexBufferLayout {
            array_stride: mem::size_of::<QuadVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                }
            ]
        }
    }
}

pub struct Quad {
    pub points: ((f32,f32),(f32,f32)),
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl Quad {
    pub fn new(device: &Device, top_left: (f32,f32), bottom_right: (f32,f32)) -> Self {
        let indices: &[u16] = &[
            0, 1, 2,
            0, 2, 3
        ];
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: BufferUsages::INDEX,
            }
        );

        let vertex_buffer = Quad::createVertexBuffer(device, &top_left, &bottom_right);

        Self {
            points: (top_left, bottom_right),
            vertex_buffer,
            index_buffer
        }
    }

    pub fn createVertexBuffer(device: &Device, top_left: &(f32,f32), bottom_right: &(f32,f32)) -> Buffer {
        let vertices: &[QuadVertex] = &[
            QuadVertex { position: [top_left.0, top_left.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [top_left.0, bottom_right.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [bottom_right.0, bottom_right.1, 0.0], color: [1.0, 1.0, 0.0] },
            QuadVertex { position: [bottom_right.0, top_left.1, 0.0], color: [1.0, 1.0, 0.0] },
        ];

        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX
        })
    }

    pub fn render<'a, 'b>(&'a self, render_pass: &mut RenderPass<'b>) where 'a : 'b {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..6, 0, 0..1);
    }

    pub fn resize(&mut self, device: &Device, top_left: (f32, f32), bottom_right: (f32, f32)) {
        let vertex_buffer = Quad::createVertexBuffer(device, &top_left, &bottom_right);
        self.points = (top_left, bottom_right);

        self.vertex_buffer = vertex_buffer;
    }

    pub fn moveComponent(&mut self, device: &Device, x: f32, y: f32) {
        self.resize(device, (self.points.0.0 + x, self.points.0.1 + y), (self.points.1.0 + x, self.points.1.1 + y));
    }

    pub fn scale(&mut self) {

    }

    pub fn create_render_pipeline(state: &State) -> RenderPipeline {
        let device = &state.device;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("quad.wgsl").into()),
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
                    QuadVertex::descriptor(),
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



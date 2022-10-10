use std::f32::consts::PI;
use wgpu::{Buffer, Device, RenderPipeline};
use wgpu::util::DeviceExt;
use crate::{Shape, State};
use crate::shapes::vertex::{BasicColorVertex, Vertex};

pub struct Oval {
    pub center: (f32, f32),
    pub diameter: (f32, f32),
    pub triangle_count: u16,
    pub color: [f32; 3],

    vertex_buffer: Buffer,
    indices_buffer: Buffer,

    render_pipeline: RenderPipeline,
}

impl Oval {
    pub fn new(center: (f32, f32), diameter: (f32, f32), triangle_count: u16, color: [f32; 3], device: &Device, state: &State) -> Self {
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
                    BasicColorVertex::get_descriptor(),
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
            center,
            diameter,
            triangle_count,
            color,
            vertex_buffer: Self::generate_vertex_buffer(&center, &diameter, &triangle_count, &color, &device),
            indices_buffer: Self::generate_indices_buffer(&triangle_count, &device),
            render_pipeline,
        }
    }

    fn generate_vertex_buffer(center: &(f32, f32), diameter: &(f32, f32), triangle_count: &u16, color: &[f32; 3], device: &Device) -> Buffer {
        let mut vertices = Vec::new();

        let vertex_count = triangle_count + 2;

        for i in 0..vertex_count {
            let angle = ((PI * 2.0) / vertex_count as f32) * i as f32;

            vertices.push(BasicColorVertex { position: [angle.cos() * diameter.0 + center.0, angle.sin() * diameter.1 + center.1, 0.0], color: color.clone() });
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        vertex_buffer
    }

    fn generate_indices_buffer(triangle_count: &u16, device: &Device) -> Buffer {
        let mut indices = Vec::new();

        for i in 1..(*triangle_count + 1) {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }

        let indices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        indices_buffer
    }
}

impl Shape for Oval {
    fn get_vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    fn update_vertex_buffer(&mut self, device: &Device) {
        self.vertex_buffer = Oval::generate_vertex_buffer(&self.center, &self.diameter, &self.triangle_count, &self.color, &device);
    }

    fn get_indices_buffer(&self) -> &Buffer {
        &self.indices_buffer
    }

    fn update_indices_buffer(&mut self, device: &Device) {
        self.indices_buffer = Oval::generate_indices_buffer(&self.triangle_count, &device);
    }

    fn get_number_indices(&self) -> u32 {
        self.triangle_count as u32 * 3
    }

    fn get_render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}


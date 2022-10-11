use wgpu::{Buffer, BufferAddress, Device, IndexFormat, RenderPass, RenderPipeline, VertexBufferLayout};
use wgpu::util::DeviceExt;
use crate::{Shape, State};
use crate::shapes::vertex::{BasicColorVertex, Vertex};
use crate::svg::Line;

pub struct LineSvg {
    lines: Vec<Line>,
    color:[f32;3],

    vertex_buffer: Buffer,
    indices_buffer: Buffer,

    render_pipeline: RenderPipeline,

    num_indices: u32,
}

impl LineSvg {
    pub fn new(lines: Vec<Line>, color: [f32; 3], device: &Device, state: &State) -> Self {
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
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: None,
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

        let mut num_indices: u32 = 0;

        let vertex_buffer = Self::generate_vertex_buffer(&lines, &color, &device);
        let indices_buffer = Self::generate_indices_buffer(&lines, &mut num_indices, &device);

        Self {

            lines,
            color,
            vertex_buffer,
            indices_buffer,
            render_pipeline,
            num_indices
        }
    }

    fn generate_vertex_buffer(lines: &Vec<Line>, color: &[f32; 3], device: &Device) -> Buffer {
        let mut vertices: Vec<BasicColorVertex> = Vec::new();

        vertices.push(BasicColorVertex {position: [-1.0,1.0,0.0], color: color.clone()});

        for line in lines {
            vertices.push(BasicColorVertex {position: [line.point1.0, line.point1.1, 0.0], color: color.clone() });
            vertices.push(BasicColorVertex {position: [line.point2.0, line.point2.1, 0.0], color: color.clone() });
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        vertex_buffer
    }

    fn generate_indices_buffer(lines: &Vec<Line>, num_indices: &mut u32, device: &Device) -> Buffer {

        let mut indices: Vec<u16> = Vec::new();
        for i in 0..lines.len() * 2 {
            indices.push(i as u16 + 1);
            if i % 2 == 1 {
                indices.push(0);
            }
        }

        *num_indices = indices.len() as u32;

        let indices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        indices_buffer
    }
}

impl Shape for LineSvg {


    fn get_vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    fn update_vertex_buffer(&mut self, device: &Device) {
        self.vertex_buffer = LineSvg::generate_vertex_buffer(&self.lines, &self.color, &device);
    }

    fn get_indices_buffer(&self) -> &Buffer {
        &self.indices_buffer
    }

    fn update_indices_buffer(&mut self, device: &Device) {
        self.indices_buffer = LineSvg::generate_indices_buffer(&self.lines, &mut self.num_indices, &device);
    }

    fn get_number_indices(&self) -> u32 {
        self.num_indices
    }

    fn get_render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }

    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(self.get_render_pipeline());

        render_pass.set_vertex_buffer(0, self.get_vertex_buffer().slice(..));
        render_pass.set_index_buffer(self.get_indices_buffer().slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.get_number_indices(), 0, 0..1);
    }
}


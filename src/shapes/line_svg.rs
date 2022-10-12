use ttf_parser::gdef::GlyphClass::Component;
use wgpu::{BindGroup, BindGroupLayout, BindGroupLayoutEntry, Buffer, BufferAddress, Device, IndexFormat, RenderPass, RenderPipeline, SurfaceConfiguration, VertexBufferLayout};
use wgpu::util::DeviceExt;
use crate::{Line, Shape, State};
use crate::components::component::ComponentUtils;
use crate::shapes::vertex::{BasicColorVertex, Vertex};

pub struct LineSvg {
    lines: Vec<Line>,
    color:[f32;3],

    parent_top_left: (f32,f32),
    parent_bottom_right: (f32,f32),

    vertex_buffer: Buffer,
    indices_buffer: Buffer,
    uniform_bind_group: BindGroup,

    uniform_layout: BindGroupLayout,

    len_buffer: Buffer,
    screen_buffer: Buffer,

    render_pipeline: RenderPipeline,

    num_indices: u32,
}


impl LineSvg {
    pub fn new(lines: Vec<Line>, color: [f32; 3], parent_top_left: (f32,f32), parent_bottom_right: (f32,f32), device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("../golygon.wgsl").into()),
        });

        let len_uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[lines.len()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let screen_uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[config.width as f32, config.height as f32]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }

            ],
            label: Some("uniform_bind_group_layout"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: Self::generate_line_uniform(&lines, parent_top_left, parent_bottom_right, config, device).as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: len_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: screen_uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_bind_group_layout
                ],
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
                    format: config.format,
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

        let vertex_buffer = Self::generate_vertex_buffer(&lines, parent_top_left, parent_bottom_right, &color, config, device);
        let indices_buffer = Self::generate_indices_buffer(&lines, &mut num_indices, &device);

        Self {
            lines,
            color,
            parent_top_left,
            parent_bottom_right,
            vertex_buffer,
            indices_buffer,
            uniform_bind_group,
            uniform_layout: uniform_bind_group_layout,
            len_buffer: len_uniform_buffer,
            screen_buffer: screen_uniform_buffer,
            render_pipeline,
            num_indices
        }
    }

    fn generate_line_uniform(lines: &Vec<Line>, parent_top_left: (f32,f32), parent_bottom_right: (f32,f32), config: &SurfaceConfiguration, device: &Device) -> Buffer {
        let mut abs_lines: Vec<Line> = Vec::new();

        for line in lines {
            let mut point1 = ComponentUtils::calculate_absolute_point_from_relative_view_points(parent_top_left, parent_bottom_right, (line.x1, line.y1));
            let mut point2 = ComponentUtils::calculate_absolute_point_from_relative_view_points(parent_top_left, parent_bottom_right, (line.x2, line.y2));

            abs_lines.push(Line {x1: point1.0,y1: point1.1, x2: point2.0, y2: point2.1});
        }

        let line_uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&abs_lines),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        line_uniform_buffer
    }

    fn generate_vertex_buffer(lines: &Vec<Line>, parent_top_left: (f32,f32), parent_bottom_right: (f32,f32), color: &[f32; 3], config: &SurfaceConfiguration, device: &Device) -> Buffer {
        let mut vertices: Vec<BasicColorVertex> = Vec::new();

        vertices.push(BasicColorVertex {position: [-1.0,1.0,0.0], color: color.clone()});

        for line in lines {

            let mut point1 = ComponentUtils::calculate_absolute_point_from_relative_view_points(parent_top_left, parent_bottom_right, (line.x1, line.y1));
            let mut point2 = ComponentUtils::calculate_absolute_point_from_relative_view_points(parent_top_left, parent_bottom_right, (line.x2, line.y2));

            vertices.push(BasicColorVertex {position: [point1.0, point1.1, 0.0], color: color.clone() });
            vertices.push(BasicColorVertex {position: [point2.0, point2.1, 0.0], color: color.clone() });
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

    pub fn resize(&mut self, parent_top_left: (f32,f32), parent_bottom_right: (f32,f32), config: &SurfaceConfiguration, device: &Device) {
        self.parent_top_left = parent_top_left;
        self.parent_bottom_right = parent_bottom_right;

        self.update_vertex_buffer(config,device);


        self.uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.uniform_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: Self::generate_line_uniform(&self.lines, parent_top_left, parent_bottom_right, config, device).as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.len_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.screen_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });
    }

    fn update_vertex_buffer(&mut self, config: &SurfaceConfiguration, device: &Device) {
        self.vertex_buffer = LineSvg::generate_vertex_buffer(&self.lines, self.parent_top_left, self.parent_bottom_right, &self.color, config, device);
    }

    pub fn update_screen_uniform(&mut self, config: &SurfaceConfiguration, device: &Device) {

        let screen_uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[config.width as f32, config.height as f32]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        self.uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.uniform_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: Self::generate_line_uniform(&self.lines, self.parent_top_left, self.parent_bottom_right, config, device).as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.len_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: screen_uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });
    }
}

impl Shape for LineSvg {


    fn get_vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    fn update_vertex_buffer(&mut self, device: &Device) {
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

    fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(self.get_render_pipeline());

        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.get_vertex_buffer().slice(..));
        render_pass.set_index_buffer(self.get_indices_buffer().slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.get_number_indices(), 0, 0..1);
    }
}


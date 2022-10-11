extern crate core;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
};
use crate::components::clickable::ClickableComponent;
use crate::components::component::Component;
use crate::components::layout::LayoutComponent;

use crate::shape::Shape;
use crate::shapes::line_svg::LineSvg;
use crate::shapes::oval::Oval;
use crate::shapes::shape;
use crate::svg::Line;

mod texture;
mod shapes;
mod components;
mod svg;

pub struct State {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    last_mouse_position: (f32, f32),

    root: Option<LayoutComponent>,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        surface.configure(&device, &config);

        Self {
            root: None,
            surface,
            device,
            queue,
            config,
            size,
            last_mouse_position: (0.0, 0.0),
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        match _event {
            WindowEvent::CursorMoved { position, .. } => {
                self.last_mouse_position = ((position.x as f32 / self.config.width as f32 * 2.0) - 1.0,
                                            ((position.y as f32 / self.config.height as f32 * 2.0) - 1.0) * -1.0);
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                match &mut self.root {
                    None => {}
                    Some(root) => {
                        root.on_click(self.last_mouse_position);
                        root.resize((0.0, 1.0), (1.0, -1.0));
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        use components::plain::PlainComponent;

        if self.root.is_none() {
            let mut layout_component = LayoutComponent::new(
                (-0.7, 0.7),
                (0.7, -0.7));

            let mut layout_component2 = LayoutComponent::new(
                (-1.0, 0.7),
                (1.0, -1.0));

            let mut layout_component3 = LayoutComponent::new(
                (0.0, 1.0),
                (1.0, -1.0));

            let plain_component = PlainComponent::new(
                (-1.0, 1.0),
                (1.0, 0.7),
                [1.0, 1.0, 0.0],
                &self.device,
                &self.config,
            );

            let plain_component2 = PlainComponent::new(
                (-1.0, 1.0),
                (0.0, -1.0),
                [1.0, 0.0, 0.0],
                &self.device,
                &self.config,
            );

            let plain_component3 = PlainComponent::new(
                (-1.0, 1.0),
                (1.0, 0.0),
                [0.0, 1.0, 0.0],
                &self.device,
                &self.config,
            );
            let clickable_component = ClickableComponent::new(
                (-1.0, 0.0),
                (1.0, -1.0),
                [0.0, 0.0, 1.0],
                &self.device,
                &self.config);

            layout_component3.add_component(Box::new(plain_component3));
            layout_component3.add_component(Box::new(clickable_component));

            layout_component2.add_component(Box::new(layout_component3));
            layout_component2.add_component(Box::new(plain_component2));

            layout_component.add_component(Box::new(layout_component2));
            layout_component.add_component(Box::new(plain_component));

            self.root = Some(layout_component);
        }

        let oval = Oval::new((1.0, 1.0), (0.1, 0.1), 64, [1.0, 0.1, 0.1], &self.device, &self);
        let oval1 = Oval::new((-1.0, 1.0), (0.5, 0.1), 64, [0.1, 1.0, 0.1], &self.device, &self);
        let oval2 = Oval::new((1.0, -1.0), (0.5, 0.1), 64, [0.1, 0.1, 1.0], &self.device, &self);
        let oval3 = Oval::new((-1.0, -1.0), (0.1, 0.1), 64, [1.0, 1.0, 0.1], &self.device, &self);

        use self::svg::Svg;
        let svg = Svg::new("src/font2.ttf");

        let line = LineSvg::new(svg.get_lines('A'), [1.0,0.0,0.0], &self.device, &self);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        },
                    })
                ],
                depth_stencil_attachment: None,
            });

            //oval.draw(&mut render_pass);
            //oval1.draw(&mut render_pass);
            //oval2.draw(&mut render_pass);
            //oval3.draw(&mut render_pass);

            line.draw(&mut render_pass);

            //quadrat.draw(&mut render_pass);
            //quadrat2.draw(&mut render_pass);

            if self.root.is_some()
            {
                //self.root.as_mut().unwrap().render(&(-1.0, 1.0), &(1.0, -1.0), &mut render_pass, &self.device, &self.config);
            }
        }

        /*
        let render_pipeline = Quad::create_render_pipeline(&self);
        let quad2 = Quad::new(&self.device, (-0.9,0.9), (0.3,-0.9));
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
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
            quad2.render(&mut render_pass);
        }
         */

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !state.input(event) { // UPDATED!
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                }
                _ => {}
            }
        }
        _ => {}
    });
}
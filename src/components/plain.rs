use wgpu::{Device, RenderPass, SurfaceConfiguration};
use crate::{Shape};
use crate::components::component::ComponentBasicResizeData;
use super::super::shapes::quad::Quad;
use super::component::{ComponentUtils, Component};

pub struct PlainComponent {
    basic: ComponentBasicResizeData,
    color: [f32; 3],

    quad: Quad,
}

impl<'a> PlainComponent {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32), color: [f32; 3], device: &Device, config: &SurfaceConfiguration) -> Self {
        Self {
            basic: ComponentBasicResizeData {
                top_left,
                bottom_right,
                needs_resize: true,
            },
            color,
            quad: Quad::new(top_left,
                            bottom_right,
                            color,
                            device,
                            config),
        }
    }
}

impl Component for PlainComponent {
    fn render<'a>(&'a mut self, parent_top_left: &(f32, f32), parent_bottom_right: &(f32, f32), render_pass: &mut RenderPass<'a>, device: &Device, config: &SurfaceConfiguration) {
        if self.basic.needs_resize {
            let (absolute_top_left, absolut_bottom_right) = ComponentUtils::calculate_absolute_from_relative_view_points(parent_top_left.clone(),
                                                                                                                         parent_bottom_right.clone(),
                                                                                                                         self.basic.top_left,
                                                                                                                         self.basic.bottom_right);

            self.quad = Quad::new(absolute_top_left,
                                  absolut_bottom_right,
                                  self.color,
                                  device,
                                  config);

            self.basic.needs_resize = false;
        }

        self.quad.draw(render_pass);
    }

    fn get_top_left(&self) -> (f32, f32) {
        self.basic.top_left
    }

    fn get_bottom_right(&self) -> (f32, f32) {
        self.basic.bottom_right
    }

    fn resize(&mut self, new_box_top_left: (f32, f32), new_box_bottom_right: (f32, f32)) {
        self.basic.top_left = new_box_top_left;
        self.basic.bottom_right = new_box_bottom_right;

        self.on_resize();
    }

    fn on_resize(&mut self) {
        self.basic.needs_resize = true;
    }
}
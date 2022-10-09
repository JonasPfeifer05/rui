use wgpu::{Device, RenderPass};
use crate::{LayoutComponent, Shape, State};
use super::super::shapes::quad::Quad;
use super::component::{ComponentUtils, Component};

pub struct PlainComponent {
    top_left: (f32,f32),
    bottom_right: (f32,f32),

    quad: Quad,
}

impl<'a> PlainComponent {
    pub fn new(parent: &LayoutComponent, box_top_left: (f32,f32), box_bottom_right: (f32, f32), color: [f32; 3], device: &Device, state: &State) -> Self {
        let (top_left, bottom_right) = ComponentUtils::calculate_absolute_from_relative_view_points(parent.get_top_left(),
                                                                                                    parent.get_bottom_right(),
                                                                                                    box_top_left,
                                                                                                    box_bottom_right);

        let quad = Quad::new(top_left,
                             bottom_right,
                             color,
                             device,
                             state);

        Self {
            quad,
            top_left,
            bottom_right,
        }
    }
}

impl Component for PlainComponent {
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        self.quad.draw(render_pass);
    }

    fn get_top_left(&self) -> (f32, f32) {
        self.top_left
    }

    fn get_bottom_right(&self) -> (f32, f32) {
        self.bottom_right
    }

    fn resize(&mut self, new_box_top_left: (f32, f32), new_box_bottom_right: (f32, f32)) {

    }

    fn on_resize(&mut self, new_parent_top_left: (f32, f32), new_parent_bottom_right: (f32, f32)) {

    }
}
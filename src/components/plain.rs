use wgpu::{Device, RenderPass};
use crate::{LayoutComponent, Shape, State};
use super::super::shapes::quad::Quad;
use super::component::{ComponentUtils, Component};

pub struct PlainComponent {
    top_left: (f32, f32),
    bottom_right: (f32, f32),
    color: [f32; 3],

    quad: Quad,
}

impl<'a> PlainComponent {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32), color: [f32; 3], state: &State) -> Self {
        Self {
            top_left,
            bottom_right,
            color,
            quad: Quad::new(top_left,
                            bottom_right,
                            color,
                            &state.device,
                            state),
        }
    }
}

impl Component for PlainComponent {
    fn render<'a>(&'a mut self, parent_top_left: &(f32, f32), parent_bottom_right: &(f32, f32), render_pass: &mut RenderPass<'a>, state: &State) {
        let (absolute_top_left, absolut_bottom_right) = ComponentUtils::calculate_absolute_from_relative_view_points(parent_top_left.clone(),
                                                                                                                     parent_bottom_right.clone(),
                                                                                                                     self.top_left,
                                                                                                                     self.bottom_right);

        self.quad = Quad::new(absolute_top_left,
                              absolut_bottom_right,
                              self.color,
                              &state.device,
                              state);


        self.quad.draw(render_pass);
    }

    fn get_top_left(&self) -> (f32, f32) {
        self.top_left
    }

    fn get_bottom_right(&self) -> (f32, f32) {
        self.bottom_right
    }

    fn resize(&mut self, new_box_top_left: (f32, f32), new_box_bottom_right: (f32, f32)) {}

    fn on_resize(&mut self, new_parent_top_left: (f32, f32), new_parent_bottom_right: (f32, f32)) {}
}
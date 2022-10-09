use wgpu::RenderPass;

use super::component::{ComponentUtils, Component};

pub struct LayoutComponent {
    top_left: (f32, f32),
    bottom_right: (f32, f32),

    components: Vec<Box<dyn Component>>,
}

impl LayoutComponent {
    pub fn new(parent: Option<&LayoutComponent>, box_top_left: (f32, f32), box_bottom_right: (f32, f32)) -> Self {
        let (top_left, bottom_right) = match parent {
            None => {
                ComponentUtils::calculate_absolute_from_relative_view_points((-1.0, 1.0),
                                                                             (1.0, -1.0),
                                                                             box_top_left,
                                                                             box_bottom_right)
            }
            Some(parent) => {
                ComponentUtils::calculate_absolute_from_relative_view_points(parent.get_top_left(),
                                                                             parent.get_bottom_right(),
                                                                             box_top_left,
                                                                             box_bottom_right)
            }
        };


        LayoutComponent {
            top_left,
            bottom_right,
            components: vec![],
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
}

impl Component for LayoutComponent {
    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        for i in 0..self.components.len() {
            self.components.get(i).unwrap().as_ref().render(render_pass);
        }
    }

    fn get_top_left(&self) -> (f32, f32) {
        self.top_left
    }

    fn get_bottom_right(&self) -> (f32, f32) {
        self.bottom_right
    }

    fn on_click(&self, position: (f32, f32)) {
        for i in 0..self.components.len() {
            let comp = self.components.get(i).unwrap().as_ref();
            if comp.in_bound(position) {
                comp.on_click(position);
            }
        }
    }

    fn resize(&mut self, new_box_top_left: (f32, f32), new_box_bottom_right: (f32, f32)) {

    }

    fn on_resize(&mut self, new_parent_top_left: (f32, f32), new_parent_bottom_right: (f32, f32)) {

    }
}
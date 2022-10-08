

use wgpu::RenderPass;

use super::component::{ComponentUtils, Component};

pub struct LayoutComponent<'a> {
    top_left: (f32,f32),
    bottom_right: (f32,f32),

    components: Vec<Box<&'a dyn Component>>,
}

impl<'a> LayoutComponent<'a> {
    pub fn new(parent_top_left: (f32, f32), parent_bottom_right: (f32, f32), box_top_left: (f32, f32), box_bottom_right: (f32, f32)) -> Self {
        let (top_left, bottom_right) = ComponentUtils::calculate_absolute_from_relative_view_points(parent_top_left,
                                                                                                    parent_bottom_right,
                                                                                                    box_top_left,
                                                                                                    box_bottom_right);

        LayoutComponent {
            top_left,
            bottom_right,
            components: vec![]
        }
    }

    pub fn add_component(&mut self, component: Box<&'a dyn Component>) {
        self.components.push(component);
    }
}

impl<'b> Component for LayoutComponent<'_> {
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
}
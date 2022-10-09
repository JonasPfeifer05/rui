use wgpu::{Device, RenderPass, SurfaceConfiguration};
use super::component::{ComponentUtils, Component};

pub struct LayoutComponent {
    top_left: (f32, f32),
    bottom_right: (f32, f32),

    components: Vec<Box<dyn Component>>,
}

impl LayoutComponent {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32)) -> Self {
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
    fn render<'a>(&'a mut self, parent_top_left: &(f32, f32), parent_bottom_right: &(f32, f32), render_pass: &mut RenderPass<'a>, device: &Device, config: &SurfaceConfiguration) {
        let (absolute_top_left, absolut_bottom_right) = ComponentUtils::calculate_absolute_from_relative_view_points(parent_top_left.clone(),
                                                                                                                     parent_bottom_right.clone(),
                                                                                                                     self.top_left,
                                                                                                                     self.bottom_right);

        for x in self.components.iter_mut() {
            x.render(&absolute_top_left, &absolut_bottom_right, render_pass, device, config);
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

    fn resize(&mut self, new_box_top_left: (f32, f32), new_box_bottom_right: (f32, f32)) {}

    fn on_resize(&mut self, new_parent_top_left: (f32, f32), new_parent_bottom_right: (f32, f32)) {}
}
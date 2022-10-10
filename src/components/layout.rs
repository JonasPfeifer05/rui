use wgpu::{Device, RenderPass, SurfaceConfiguration};
use crate::components::component::ComponentBasicData;
use super::component::{ComponentUtils, Component};

pub struct LayoutComponent {
    basic: ComponentBasicData,

    components: Vec<Box<dyn Component>>,
}

impl LayoutComponent {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32)) -> Self {
        LayoutComponent {
            basic: ComponentBasicData {
                top_left,
                bottom_right,
            },
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
                                                                                                                     self.basic.top_left,
                                                                                                                     self.basic.bottom_right);

        for comp in self.components.iter_mut() {
            comp.render(&absolute_top_left, &absolut_bottom_right, render_pass, device, config);
        }
    }

    fn get_top_left(&self) -> (f32, f32) {
        self.basic.top_left
    }

    fn get_bottom_right(&self) -> (f32, f32) {
        self.basic.bottom_right
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

        self.basic.top_left = new_box_top_left;
        self.basic.bottom_right = new_box_bottom_right;

        self.on_resize();
    }

    fn on_resize(&mut self) {

        for comp in self.components.iter_mut() {
            comp.on_resize()
        }
    }
}
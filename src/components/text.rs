use wgpu::{Device, RenderPass, SurfaceConfiguration};
use crate::{Line, LineSvg, Shape};
use crate::components::component::ComponentBasicResizeData;
use crate::svg::Svg;
use super::super::shapes::quad::Quad;
use super::component::{ComponentUtils, Component};

pub struct TextComponent {
    basic: ComponentBasicResizeData,
    color: [f32; 3],

    svg: LineSvg,
}

impl<'a> TextComponent {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32), svg: Svg, letter: char, color: [f32; 3], device: &Device, config: &SurfaceConfiguration) -> Self {
        let lines = svg.get_lines(letter);
        let mut lines_scaled = Vec::new();

        for line in lines {
            let point1 = ComponentUtils::calculate_absolute_point_from_relative_view_points(top_left, bottom_right, (line.x1, line.y1));
            let point2 = ComponentUtils::calculate_absolute_point_from_relative_view_points(top_left, bottom_right, (line.x2, line.y2));

            lines_scaled.push(Line {x1: point1.0, y1: point1.1, x2: point2.0, y2: point2.1});
        }


        Self {
            basic: ComponentBasicResizeData {
                top_left,
                bottom_right,
                needs_resize: true,
            },
            color,
            svg: LineSvg::new(lines_scaled, color, top_left, bottom_right, device, config),
        }
    }
}

impl Component for TextComponent {
    fn render<'a>(&'a mut self, parent_top_left: &(f32, f32), parent_bottom_right: &(f32, f32), render_pass: &mut RenderPass<'a>, device: &Device, config: &SurfaceConfiguration) {
        self.svg.update_screen_uniform(config, device);

        if self.basic.needs_resize {
            self.svg.resize(parent_top_left.clone(), parent_bottom_right.clone(), config, device);
            println!("Moin");
            self.basic.needs_resize = false;
        }

        self.svg.draw(render_pass);
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

        println!("Hallo");

        self.on_resize();
    }

    fn on_resize(&mut self) {
        self.basic.needs_resize = true;
    }
}
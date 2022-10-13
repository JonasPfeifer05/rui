use wgpu::{Device, RenderPass, SurfaceConfiguration};
use crate::{Line, LineSvg, Shape};
use crate::components::component::ComponentBasicResizeData;
use crate::svg::Svg;
use super::super::shapes::quad::Quad;
use super::component::{ComponentUtils, Component};

pub struct FontStyle {
    pub(crate) size_x: f32,
    pub(crate) size_y: f32,
    pub(crate) color: [f32; 3],
}

pub struct TextComponent {
    basic: ComponentBasicResizeData,
    font_style: FontStyle,

    svg: LineSvg,
}

impl<'a> TextComponent {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32), svg: Svg, letter: char, font_style: FontStyle, device: &Device, config: &SurfaceConfiguration) -> Self {
        let lines = svg.get_lines(letter);
        let mut lines_scaled = Vec::new();

        let font_top_left = top_left;
        let mut font_bottom_right = top_left;
        font_bottom_right.0 += (bottom_right.0 - top_left.0) / (100.0 / font_style.size_x);
        font_bottom_right.1 -= (top_left.1 - bottom_right.1) / (100.0 / font_style.size_y);

        for line in lines {
            let point1 = ComponentUtils::calculate_absolute_point_from_relative_view_points(font_top_left, font_bottom_right, (line.x1, line.y1));
            let point2 = ComponentUtils::calculate_absolute_point_from_relative_view_points(font_top_left, font_bottom_right, (line.x2, line.y2));

            lines_scaled.push(Line {x1: point1.0, y1: point1.1, x2: point2.0, y2: point2.1});
        }


        Self {
            svg: LineSvg::new(lines_scaled, font_style.color, top_left, bottom_right, device, config),

            basic: ComponentBasicResizeData {
                top_left,
                bottom_right,
                needs_resize: true,
            },
            font_style,
        }
    }
}

impl Component for TextComponent {
    fn render<'a>(&'a mut self, parent_top_left: &(f32, f32), parent_bottom_right: &(f32, f32), render_pass: &mut RenderPass<'a>, device: &Device, config: &SurfaceConfiguration) {
        self.svg.update_screen_uniform(config, device);

        if self.basic.needs_resize {
            self.svg.resize(parent_top_left.clone(), parent_bottom_right.clone(), config, device);
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
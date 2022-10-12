use std::fmt::Write;
use std::fs::File;
use ttf_parser::{Face, Rect};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Line {
    pub x1: f32,
    pub y1: f32,

    pub x2: f32,
    pub y2: f32,
}

struct Builder(Vec<String>);

impl ttf_parser::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        let mut command = String::new();
        write!(&mut command, "M {} {}", x, y).unwrap();
        self.0.push(command);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let mut command = String::new();
        write!(&mut command, "L {} {}", x, y).unwrap();
        self.0.push(command);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let mut command = String::new();
        write!(&mut command, "Q {} {} {} {}", x1, y1, x, y).unwrap();
        self.0.push(command);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let mut command = String::new();
        write!(&mut command, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap();
        self.0.push(command);
    }

    fn close(&mut self) {
        self.0.push(String::from("Z "));
    }
}

pub struct Svg {
    data: Vec<u8>,
}

impl Svg {
    pub fn new(path: &str) -> Self {
        let data = std::fs::read(path).unwrap();

        Self {
            data,
        }
    }

    pub fn get_lines(&self, letter: char) -> Vec<Line> {
        let face = Face::parse(&self.data, 0).unwrap();

        let mut lines: Vec<Line> = Vec::new();

        let mut builder = Builder(Vec::new());

        let bbox = face.outline_glyph(face.glyph_index(letter).unwrap(), &mut builder).unwrap();
        let rect = face.glyph_bounding_box(face.glyph_index(letter).unwrap()).unwrap();

        let operations: Vec<String> = builder.0;

        let mut current_pos = (0.0,0.0);

        for operation in operations {
            let tokens: Vec<_> = operation.split(" ").collect();

            match tokens.get(0).unwrap() {
                &"M" => {
                    current_pos = Svg::get_position(tokens.get(1).unwrap(), tokens.get(2).unwrap());
                }
                &"L" => {
                    let old_position = current_pos.clone();
                    current_pos = Svg::get_position(tokens.get(1).unwrap(), tokens.get(2).unwrap());
                    let mut line = Line { x1: old_position.0, y1: old_position.1, x2: current_pos.0, y2: current_pos.1 };
                    Svg::zero_to_one(&mut line, &rect);
                    lines.push(line);
                }
                &"Q" => {
                    let old_position = current_pos.clone();
                    current_pos = Svg::get_position(tokens.get(3).unwrap(), tokens.get(4).unwrap());
                    let mut line = Line { x1: old_position.0, y1: old_position.1, x2: current_pos.0, y2: current_pos.1 };
                    Svg::zero_to_one(&mut line, &rect);
                    lines.push(line);}
                &"Z" => {

                }
                &err => {println!("{}",err)}
            }
        }

        lines
    }

    fn zero_to_one(line: &mut Line, rect: &Rect) {
        line.x1 /= rect.width() as f32;
        line.y1 /= rect.height() as f32;

        line.x2 /= rect.width() as f32;
        line.y2 /= rect.height() as f32;
    }

    fn get_position(x: &str, y: &str) -> (f32,f32) {
        let x_float = x.parse::<f32>().unwrap();
        let y_float = y.parse::<f32>().unwrap();

        (x_float, y_float)
    }
}


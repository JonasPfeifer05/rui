use wgpu::{Device, RenderPass, SurfaceConfiguration};

pub struct ComponentBasicData{
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),
}

pub struct ComponentBasicResizeData{
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),
    pub needs_resize: bool
}

pub trait Component {
    fn render<'a>(&'a mut self, parent_top_left: &(f32, f32), parent_bottom_right: &(f32, f32), render_pass: &mut RenderPass<'a>, device: &Device, config: &SurfaceConfiguration);
    fn get_top_left(&self) -> (f32, f32);
    fn get_bottom_right(&self) -> (f32, f32);

    fn on_click(&self, position: (f32, f32)) {}
    fn in_bound(&self, point: (f32, f32)) -> bool {
        if point.0 > self.get_top_left().0 && point.0 < self.get_bottom_right().0 {
            if point.1 < self.get_top_left().1 && point.1 > self.get_bottom_right().1 {
                return true;
            }
        }

        false
    }

    fn resize(&mut self, new_box_top_left: (f32, f32), new_box_bottom_right: (f32, f32));
    fn on_resize(&mut self);
}

pub struct ComponentUtils {}

impl ComponentUtils {
    pub fn view_to_compute(mut point: (f32, f32)) -> (f32, f32) {
        point.1 *= -1.0;

        point.0 += 1.0;
        point.1 += 1.0;

        point.0 /= 2.0;
        point.1 /= 2.0;

        point
    }

    fn compute_to_view(mut point: (f32, f32)) -> (f32, f32) {
        point.0 *= 2.0;
        point.1 *= 2.0;

        point.0 -= 1.0;
        point.1 -= 1.0;

        point.1 *= -1.0;

        point
    }

    fn calculate_absolute(parent_top_left: (f32, f32), parent_bottom_right: (f32, f32), point: (f32, f32)) -> (f32, f32) {
        let mut new_point = (0.0, 0.0);

        new_point.0 = parent_top_left.0 + (parent_bottom_right.0 - parent_top_left.0) * point.0;
        new_point.1 = parent_top_left.1 + (parent_bottom_right.1 - parent_top_left.1) * point.1;

        new_point
    }

    pub fn calculate_absolute_point_from_relative_view_points(mut parent_top_left: (f32, f32), mut parent_bottom_right: (f32, f32), mut point: (f32,f32)) -> (f32, f32) {
        parent_top_left = ComponentUtils::view_to_compute(parent_top_left);
        parent_bottom_right = ComponentUtils::view_to_compute(parent_bottom_right);

        point = ComponentUtils::view_to_compute(point);

        point = ComponentUtils::compute_to_view(ComponentUtils::calculate_absolute(parent_top_left, parent_bottom_right, point));

        point
    }

    pub fn calculate_absolute_box_from_relative_view_points(mut parent_top_left: (f32, f32), mut parent_bottom_right: (f32, f32), mut box_top_left: (f32, f32), mut box_bottom_right: (f32, f32)) -> ((f32, f32), (f32, f32)) {
        parent_top_left = ComponentUtils::view_to_compute(parent_top_left);
        parent_bottom_right = ComponentUtils::view_to_compute(parent_bottom_right);

        box_top_left = ComponentUtils::view_to_compute(box_top_left);
        box_bottom_right = ComponentUtils::view_to_compute(box_bottom_right);

        let top_left = ComponentUtils::compute_to_view(ComponentUtils::calculate_absolute(parent_top_left, parent_bottom_right, box_top_left));
        let bottom_right = ComponentUtils::compute_to_view(ComponentUtils::calculate_absolute(parent_top_left, parent_bottom_right, box_bottom_right));

        (top_left, bottom_right)
    }
}
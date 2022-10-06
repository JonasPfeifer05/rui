use wgpu::{CommandEncoder, RenderPass, RenderPipeline};
use crate::State;

trait Component {
    fn render(&self, state: &State, render_pass: &mut RenderPass, encoder: &mut CommandEncoder);
}
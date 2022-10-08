use wgpu::{Buffer, Device, IndexFormat, RenderPass, RenderPipeline, VertexBufferLayout};

pub trait Vertex {
    fn get_descriptor<'a>() -> VertexBufferLayout<'a>;
}

pub trait Shape {
    fn get_vertex_buffer(&self) -> &Buffer;
    fn update_vertex_buffer(&mut self, device: &Device);

    fn get_indices_buffer(&self) -> &Buffer;
    fn update_indices_buffer(&mut self, device: &Device);

    fn get_number_indices(&self) -> u32;

    fn get_render_pipeline(&self) -> &RenderPipeline;

    fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(self.get_render_pipeline());

        render_pass.set_vertex_buffer(0, self.get_vertex_buffer().slice(..));
        render_pass.set_index_buffer(self.get_indices_buffer().slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.get_number_indices(), 0, 0..1);
    }
}
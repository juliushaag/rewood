use super::renderer::RenderStage;



// This stage clears the screen and renders all the necessary geometries as the foundation
pub struct GeomStage {

}

impl RenderStage for GeomStage {

  fn render(&mut self, encoder : & mut wgpu::CommandEncoder, view : &wgpu::TextureView) {

  }
}
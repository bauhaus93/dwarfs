use graphics::{ ShaderProgram, GraphicsError };
use world::Camera;

pub trait Renderable {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError>;
}

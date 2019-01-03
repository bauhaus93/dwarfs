use graphics::{ ShaderProgram, GraphicsError };
use super::Camera;

pub trait Renderable {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError>;
}

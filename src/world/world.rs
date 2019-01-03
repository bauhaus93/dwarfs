use glm::Vector3;

use application::ApplicationError;
use graphics::{ ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError, mesh::create_plane };
use super::{ Camera, Object, Updatable, Renderable, Positionable, Rotatable };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    plane: Object
}

impl World {
    pub fn new() -> Result<World, ApplicationError> {
        let texture_array = TextureArrayBuilder::new("resources/tex.png", (256, 256))
            .add_texture((0, 0))
            .finish()?;
        let world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            plane: Object::new(create_plane()?)
        };
        Ok(world)
    }

    pub fn render(&mut self, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.texture_array.activate();
        self.plane.render(&self.camera, shader)?;
        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) {
        self.plane.mod_rotation(Vector3::new(0., 0., 0.1));
        // self.camera.mod_position(Vector3::new(0., 0., 0.5));
    }
}

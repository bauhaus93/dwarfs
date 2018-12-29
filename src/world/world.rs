use application::ApplicationError;
use graphics::{ ShaderProgram, TextureArray, Mesh, GraphicsError, mesh::create_plane };
use super::{ Camera, Object, Updatable, Renderable };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    plane: Object
}

impl World {
    pub fn new(texture_array: TextureArray) -> Result<World, ApplicationError> {
        let world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            plane: Object::new(create_plane()?)
        };
        Ok(world)
    }

    pub fn render(&mut self, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.plane.render(&self.camera, shader)?;
        Ok(())
    }
}


impl Updatable for World {
    fn tick(&mut self, last_tick: u32) {

    }
}

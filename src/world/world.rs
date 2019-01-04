use glm::Vector3;
use gl::types::GLfloat;

use application::ApplicationError;
use graphics::{ ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use world::{ Camera, Layer, traits::{ Translatable, Updatable, Renderable } };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    layer: Layer
}

impl World {
    pub fn new() -> Result<World, ApplicationError> {
        let texture_array = TextureArrayBuilder::new("resources/tex.png", (256, 256))
            .add_texture((0, 0))
            .finish()?;
        let world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            layer: Layer::new(0, (32, 32))?
        };
        Ok(world)
    }

    pub fn move_camera(&mut self, offset: [f32; 3]) {
        self.camera.mod_position(Vector3::<GLfloat>::new(offset[0], offset[1], offset[2]));
    }

    pub fn render(&mut self, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.texture_array.activate();
        self.layer.render(&self.camera, shader)?;
        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) {
        //self.quad.mod_rotation(Vector3::new(0., 0., 0.1));
        // self.camera.mod_position(Vector3::new(0., 0., 0.5));
    }
}

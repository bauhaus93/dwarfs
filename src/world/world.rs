use std::collections::BTreeMap;
use glm::Vector3;
use gl::types::GLfloat;

use application::ApplicationError;
use graphics::{ ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use world::{ Camera, Layer, traits::{ Translatable, Rotatable, Updatable, Renderable } };
use world::noise::{ Noise, OctavedNoise };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    height_noise: OctavedNoise,
    layers: BTreeMap<i32, Layer>,
}

impl World {
    pub fn new() -> Result<World, ApplicationError> {
        let texture_array = TextureArrayBuilder::new("resources/atlas.png", (32, 32))
            .add_texture((0, 0))
            .finish()?;

        let mut height_noise = OctavedNoise::default();
        height_noise.set_scale(8e-3);
        height_noise.set_roughness(1e+3);
        height_noise.set_range((0., 5.));

        let mut world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            height_noise: height_noise,
            layers: BTreeMap::new()
        };
        for level in -5..5 {
            let layer = Layer::new(level, (128, 128), &world.height_noise)?;
            world.layers.insert(level, layer);
        }
        Ok(world)
    }

    pub fn move_camera(&mut self, offset: [f32; 3]) {
        self.camera.mod_position(Vector3::<GLfloat>::new(offset[0], offset[1], offset[2]));
    }

    pub fn render(&self, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.texture_array.activate();

        for (level, layer) in self.layers.iter() {
            layer.render(&self.camera, shader)?;
        }
        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) {
    }
}

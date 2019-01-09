use glm::Vector3;
use gl::types::GLfloat;

use application::ApplicationError;
use graphics::{ ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use world::{ Camera, Layer, traits::{ Translatable, Rotatable, Updatable, Renderable } };
use world::noise::{ Noise, OctavedNoise };
use world::height_map::{ HeightMap, create_height_map };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    height_map: HeightMap,
    layers: Vec<Layer>,
}

impl World {
    pub fn new() -> Result<World, ApplicationError> {
        const TOP_LEVEL: i32 = 5;
        const LAYER_SIZE: (i32, i32) = (128, 128);
        let texture_array = TextureArrayBuilder::new("resources/atlas.png", (32, 32))
            .add_texture((0, 0))
            .finish()?;

        let mut height_noise = OctavedNoise::default();
        height_noise.set_octaves(4);
        height_noise.set_scale(8e-3);
        height_noise.set_roughness(1e+3);
        height_noise.set_range((0., 5.));
        let height_map = create_height_map(LAYER_SIZE, &height_noise);

        let mut world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            height_map: height_map,
            layers: Vec::new()
        };

        world.create_top_layer(TOP_LEVEL, LAYER_SIZE)?;
        world.create_layers(20)?;

        Ok(world)
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }


    fn create_top_layer(&mut self, top_level: i32, layer_size: (i32, i32)) -> Result<(), ApplicationError> {
        debug_assert!(self.layers.is_empty());
        let top_layer = Layer::new_top(top_level, layer_size, &self.height_map)?;
        self.layers.push(top_layer);
        Ok(())
    }

    fn create_layers(&mut self, count: i32) -> Result<(), ApplicationError> {
        debug_assert!(!self.layers.is_empty());
        for level in 0..count {
            let layer = Layer::new(&self.layers[self.layers.len() - 1], &self.height_map)?;
            self.layers.push(layer);
        }
        Ok(())
    }

    pub fn render(&self, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.texture_array.activate();

        for layer in self.layers.iter().rev() {
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


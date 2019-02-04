use std::collections::BTreeSet;
use glm::Vector3;

use application::ApplicationError;
use graphics::{ Projection, Mesh, MeshManager, ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use graphics::projection::{ create_default_orthographic, create_default_perspective };
use graphics::transformation::create_direction;
use world::{ Object, Camera, Layer, LayerCreator, WorldError, traits::{ Updatable, Renderable } };
use world::noise::{ Noise, OctavedNoise };
use world::height_map::{ HeightMap, create_height_map };
use utility::traits::{ Translatable, Rotatable, Scalable };
use utility::Float;

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    mesh_manager: MeshManager,
    top_level: i32,
    layer_creator: LayerCreator,
    layers: BTreeSet<Layer>,
    test_object: Object
}

const TEXTURE_LAYER_MUD: i32 = 0;

const TEXTURES: [[i32; 3]; 1] = [
    [0, 0, TEXTURE_LAYER_MUD]
];

impl World {
    pub fn new(top_level: i32, layer_size: [i32; 2]) -> Result<World, WorldError> {
        debug_assert!(top_level > 0);
        debug_assert!(layer_size[0] > 0 && layer_size[1] > 0);
        let texture_array = TextureArrayBuilder::new("resources/atlas.png", [32, 32])
            .add_texture([0, 0, 0])
            .finish()?;

        let mut height_noise = OctavedNoise::default();
        height_noise.set_octaves(4);
        height_noise.set_scale(8e-3);
        height_noise.set_roughness(1e+3);
        height_noise.set_range((0., 5.));
        let height_map = create_height_map(layer_size, &height_noise);

        let mut mesh_manager = MeshManager::default();
        mesh_manager.add_mesh(Mesh::from_obj("resources/obj/test.obj")?, "test");

        let mut test_object = Object::new(mesh_manager.get_mesh_rc("test")?);
        test_object.set_translation(Vector3::new(-1., -1., 1.));

        let camera = Camera::default();
        let cam_dir = create_direction(camera.get_rotation());
        info!("Camera direction = {:.2}/{:.2}/{:.2}", cam_dir.x, cam_dir.y, cam_dir.z);

        let layer_creator = LayerCreator::new(layer_size, height_map, cam_dir)?;

        let mut world = World {
            texture_array: texture_array,
            camera: camera,
            mesh_manager: mesh_manager,
            top_level: top_level,
            layer_creator: layer_creator,
            layers: BTreeSet::new(),
            test_object: test_object
        };

        for level in top_level..-5 {
            world.request_layer_creation(level);
        }

        Ok(world)
    }

    pub fn move_camera(&mut self, mut offset: Vector3<Float>) {
        let curr_height = self.camera.get_translation().z;
        match self.camera.get_projection() {
            Projection::Orthographic { .. } if curr_height > 0. => { offset.z = -curr_height; },
            Projection::Orthographic { .. } if curr_height + offset.z > 0. => { offset.z = 0.; },
            _ => {}
        }
        self.camera.mod_translation(offset);
    }

    pub fn toggle_camera_projection(&mut self) {
        match self.camera.get_projection() {
            Projection::Orthographic { .. } => {
                self.camera.set_projection(create_default_perspective());
                self.camera.set_translation(Vector3::new(-10., -10., 20.));
            },
            Projection::Perspective { .. } => {
                self.camera.set_projection(create_default_orthographic());
                self.camera.set_translation(Vector3::new(0., 0., 0.));
            }
        }
    }

    pub fn request_layer_creation(&mut self, level: i32) {
        self.layer_creator.request_layer(level);
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn render(&self, shader: &ShaderProgram) -> Result<(), WorldError> {
        self.texture_array.activate();

        self.test_object.render(&self.camera, shader)?;

        let layer_iter = match self.camera.get_translation().z {
            height if height < 0. => self.layers.iter().skip((self.top_level - self.camera.get_translation().z as i32).max(0) as usize).rev(),
            _ => self.layers.iter().skip(0).rev()
        };
        for layer in layer_iter {
            layer.render(&self.camera, shader)?;
        }
        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) {
        self.test_object.mod_rotation(Vector3::new(0., 0., 5f32.to_radians()));
    }
}


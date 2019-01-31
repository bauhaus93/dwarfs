use glm::Vector3;
use gl::types::GLfloat;

use application::ApplicationError;
use graphics::{ Projection, Mesh, MeshManager, ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use graphics::projection::{ create_default_orthographic, create_default_perspective };
use graphics::transformation::create_direction;
use world::{ Object, Camera, Layer, WorldError, traits::{ Updatable, Renderable } };
use world::noise::{ Noise, OctavedNoise };
use world::height_map::{ HeightMap, create_height_map };
use utility::traits::{ Translatable, Rotatable, Scalable };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    height_map: HeightMap,
    mesh_manager: MeshManager,
    top_level: i32,
    layer_size: [i32; 2],
    layers: Vec<Layer>,
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
        mesh_manager.add_mesh(Mesh::from_obj("resources/obj/cube.obj")?, "cube");
        mesh_manager.add_mesh(Mesh::from_obj("resources/obj/slope.obj")?, "slope");

        let test_mesh = match Mesh::from_obj("resources/obj/test.obj") {
            Ok(mesh) => mesh,
            Err(e) => { return Err(WorldError::from(GraphicsError::from(e))); }
        };
        mesh_manager.add_mesh(test_mesh, "test");
        let mut test_object = Object::new(mesh_manager.get_mesh_rc("test")?);
        test_object.set_translation(Vector3::new(1., 1., 1.));

        let mut world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            height_map: height_map,
            mesh_manager: mesh_manager,
            top_level: top_level,
            layer_size: layer_size,
            layers: Vec::new(),
            test_object: test_object
        };
        info!("Camera direction = {:?}", create_direction(world.camera.get_rotation()));

        world.extend(10)?;

        Ok(world)
    }

    pub fn move_camera(&mut self, mut offset: Vector3<f32>) {
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

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    fn extend(&mut self, count: i32) -> Result<(), WorldError> {
        debug_assert!(!self.layers.is_empty());
        let camera_direction = create_direction(self.camera.get_rotation());
        for _ in 0..count {
            let layer = Layer::new(self.top_level - self.layers.len() as i32, self.layer_size,  &self.height_map, &self.mesh_manager, camera_direction)?;
            self.layers.push(layer);
        }
        Ok(())
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


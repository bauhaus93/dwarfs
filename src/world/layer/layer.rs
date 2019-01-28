use std::rc::Rc;
use std::collections::{ HashMap };
use std::time;

use gl::types::GLfloat;
use glm::Vector3;

use application::ApplicationError;

use graphics::{ Mesh, MeshManager, Triangle, ShaderProgram, GraphicsError };
use world::{ Camera, Object, Noise, WorldError, traits::Renderable };
use world::height_map::HeightMap;
use utility::traits::{ Translatable, Rotatable, Scalable };
use super::{ Field, create_mesh };

type FieldMap = HashMap<(i32, i32), Field>;

pub struct Layer {
    object: Object,
    level: i32,
    size: (i32, i32),
    fields: FieldMap,
}

impl Layer {
    pub fn new(upper_layer: &Layer, height_map: &HeightMap, mesh_manager: &MeshManager) -> Result<Self, WorldError> {
        let level = upper_layer.level - 1;
        let size = upper_layer.size;
        debug!("Creating new layer, level = {}, size = {}x{}", level, size.0, size.1);
        debug_assert!(size.0 >= 0 && size.1 >= 0);

        let fields = create_default_field_map(level, height_map);
        let mesh = create_mesh(&fields, mesh_manager, Some(&upper_layer.fields))?;
        debug!("Layer mesh vertex count: {}", mesh.get_vertex_count());
        let mut object = Object::new(Rc::new(mesh));
        object.set_translation(Vector3::new(0., 0., level as f32));
        Ok(Self {
            object: object,
            level: level,
            size: size,
            fields: fields
        })
    }

    pub fn new_top(level: i32, size: (i32, i32), height_map: &HeightMap, mesh_manager: &MeshManager) -> Result<Self, WorldError> {
        debug!("Creating new top layer, level = {}, size = {}x{}", level, size.0, size.1);
        debug_assert!(size.0 >= 0 && size.1 >= 0);

        let fields = create_default_field_map(level, height_map);
        let mesh = create_mesh(&fields, mesh_manager, None)?;
        debug!("Layer mesh vertex count: {}", mesh.get_vertex_count());
        let mut object = Object::new(Rc::new(mesh));
        object.set_translation(Vector3::new(0., 0., level as f32));
        Ok(Self {
            object: object,
            level: level,
            size: size,
            fields: fields
        })
    }
}

impl Renderable for Layer {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.object.render(camera, shader)
    }
}

fn create_default_field_map(level: i32, height_map: &HeightMap) -> FieldMap {
    let mut fields = FieldMap::new();
    for (pos, h) in height_map.iter() {
        if level < *h {
            fields.insert(*pos, Field::default());
        }
    }
    fields
}
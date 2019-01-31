use std::rc::Rc;
use std::collections::{ HashMap, hash_map::Entry };
use std::time;

use gl::types::GLfloat;
use glm::Vector3;

use application::ApplicationError;

use graphics::{ Mesh, MeshManager, Triangle, ShaderProgram, GraphicsError };
use world::{ Direction, Camera, Object, Noise, WorldError, traits::Renderable, DIRECTION_VECTOR };
use world::height_map::HeightMap;
use utility::traits::{ Translatable, Rotatable, Scalable };
use super::{ Field, FieldType, create_mesh };

type FieldMap = HashMap<[i32; 2], Field>;

pub struct Layer {
    object: Object,
    level: i32,
    size: [i32; 2],
    fields: FieldMap,
}

impl Layer {
    pub fn new(level: i32, size: [i32; 2], height_map: &HeightMap, mesh_manager: &MeshManager, camera_direction: Vector3<f32>) -> Result<Self, WorldError> {
        debug!("Creating new layer, level = {}, size = {}x{}", level, size[0], size[1]);
        debug_assert!(size[0]>= 0 && size[1] >= 0);

        let fields = create_default_field_map(level, size, height_map);
        let mesh = create_mesh(&fields, mesh_manager, camera_direction)?;
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

fn create_default_field_map(level: i32, size: [i32; 2], height_map: &HeightMap) -> FieldMap {
    let mut fields = FieldMap::new();
    for (pos, h) in height_map.iter() {
        if level < *h {
            fields.insert(*pos, Field::default());
        }
    }
    if level >= 0 {
        let mut slope_fields = FieldMap::new();
        for (pos, field) in fields.iter() {
            for (dir, offset) in DIRECTION_VECTOR.iter().take(4) {
                let nb_pos = [pos[0] + offset[0], pos[1] + offset[1]];
                match fields.get(&nb_pos) {
                    None if nb_pos[0] >= 0 && nb_pos[0] < size[0] && nb_pos[1] >= 0 && nb_pos[1] < size[1] => {
                        let mut field = Field::default();
                        field.set_type(FieldType::SLOPE(*dir));
                        slope_fields.insert(nb_pos, field);
                    },
                    _ => {}
                }
            }
        }
        fields.extend(slope_fields);
    }


    fields
}
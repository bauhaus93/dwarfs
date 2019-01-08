use std::collections::HashMap;
use std::time;

use gl::types::GLfloat;
use glm::Vector3;

use application::ApplicationError;
use graphics::{ Mesh, MeshBuilder, ShaderProgram, GraphicsError };
use graphics::mesh::{ Quad };
use world::{ Camera, Object, Noise, traits::{ Renderable, Translatable, Scalable } };
use world::height_map::HeightMap;
use super::Field;

type FieldMap = HashMap<(i32, i32), Field>;

pub struct Layer {
    object: Object,
    level: i32,
    size: (i32, i32),
    fields: FieldMap,
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

impl Layer {
    pub fn new(upper_layer: &Layer, height_map: &HeightMap) -> Result<Self, ApplicationError> {
        let level = upper_layer.level - 1;
        let size = upper_layer.size;
        debug!("Creating new layer, level = {}, size = {}x{}", level, size.0, size.1);
        debug_assert!(size.0 >= 0 && size.1 >= 0);

        let fields = create_default_field_map(level, height_map);
        let mesh = create_mesh(&fields, Some(&upper_layer.fields))?;
        debug!("Layer mesh vertex count: {}", mesh.get_vertex_count());
        let mut object = Object::new(mesh);
        object.set_position(Vector3::new(0., 0., level as f32));
        Ok(Self {
            object: object,
            level: level,
            size: size,
            fields: fields
        })
    }

    pub fn new_top(level: i32, size: (i32, i32), height_map: &HeightMap) -> Result<Self, ApplicationError> {
        debug!("Creating new top layer, level = {}, size = {}x{}", level, size.0, size.1);
        debug_assert!(size.0 >= 0 && size.1 >= 0);

        let fields = create_default_field_map(level, height_map);
        let mesh = create_mesh(&fields, None)?;
        debug!("Layer mesh vertex count: {}", mesh.get_vertex_count());
        let mut object = Object::new(mesh);
        object.set_position(Vector3::new(0., 0., level as f32));
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

fn create_mesh(fields: &HashMap<(i32, i32), Field>, upper_fields: Option<&HashMap<(i32, i32), Field>>) -> Result<Mesh, GraphicsError> {
    let mut builder = MeshBuilder::new();
    for (pos, field) in fields {
        let mut has_top = false;
        match upper_fields {
            Some(upper) => {
                match upper.get(pos) {
                    None => { has_top = true; },
                    _ => {}
                }
            },
            _ => { has_top = true; }
        }
        if has_top {
            let mut top_quad = Quad::default();
            top_quad.translate(Vector3::new(pos.0 as GLfloat, pos.1 as GLfloat, 0.5));
            builder = builder.add_quad(top_quad);
        }

        match fields.get(&(pos.0, pos.1 - 1)) {
            None => {
                let mut right_quad = Quad::default();
                right_quad.rotate(Vector3::new(90f32.to_radians() as GLfloat, 0., 0.));
                right_quad.translate(Vector3::new(pos.0 as GLfloat, pos.1 as GLfloat - 0.5, 0.));
                right_quad.cycle_uvs(1);
                builder = builder.add_quad(right_quad);
            },
            _ => {}
        }
        
        match fields.get(&(pos.0 - 1, pos.1)) {
            None => {
                let mut left_quad = Quad::default();
                left_quad.rotate(Vector3::new(0., -90f32.to_radians() as GLfloat, 0.));
                left_quad.translate(Vector3::new(pos.0 as GLfloat - 0.5, pos.1 as GLfloat, 0.));
                left_quad.cycle_uvs(2);
                builder = builder.add_quad(left_quad);
            },
            _ => {}
        };
    }
    Ok(builder.finish()?)
}


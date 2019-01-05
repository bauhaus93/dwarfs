use std::collections::HashMap;

use gl::types::GLfloat;
use glm::Vector3;

use application::ApplicationError;
use graphics::{ Mesh, MeshBuilder, ShaderProgram, GraphicsError };
use graphics::mesh::{ Quad };
use world::{ Camera, Object, Noise, traits::{ Renderable, Translatable, Scalable } };
use super::Field;

pub struct Layer {
    object: Object,
    fields: HashMap<(u32, u32), Field>,
}

impl Layer {
    pub fn new(level: i32, size: (u32, u32), height_noise: &Noise) -> Result<Self, ApplicationError> {
        debug!("Creating layer, level = {}, size = {}x{}", level, size.0, size.1);
        let mut fields = HashMap::new();
        for y in 0..size.1 {
            for x in 0..size.0 {
                if height_noise.get_noise((x as f32, y as f32)) > level as f32 {
                    fields.insert((x, y), Field::default());
                }
            }
        }
        let mesh = create_mesh(&fields)?;
        debug!("Layer mesh vertex count: {}", mesh.get_vertex_count());
        let mut object = Object::new(mesh);
        object.set_position(Vector3::new(0., 0., level as f32));
        Ok(Self {
            object: object,
            fields: fields
        })
    }
}

impl Renderable for Layer {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.object.render(camera, shader)
    }
}

fn create_mesh(fields: &HashMap<(u32, u32), Field>) -> Result<Mesh, GraphicsError> {
    let mut builder = MeshBuilder::new();
    for (pos, field) in fields {        
        let mut top_quad = Quad::default();
        top_quad.translate(Vector3::new(pos.0 as GLfloat, pos.1 as GLfloat, 0.5));
        builder = builder.add_quad(top_quad);
        match fields.get(&(pos.0, pos.1 - 1)) {
            Some(_) => {},
            None => {
                let mut right_quad = Quad::default();
                right_quad.rotate(Vector3::new(90f32.to_radians() as GLfloat, 0., 0.));
                right_quad.translate(Vector3::new(pos.0 as GLfloat, pos.1 as GLfloat - 0.5, 0.));
                right_quad.cycle_uvs(1);
                builder = builder.add_quad(right_quad);
            }
        }
        match fields.get(&(pos.0 - 1, pos.1)) {
            Some(_) => {},
            None => {
                let mut left_quad = Quad::default();
                left_quad.rotate(Vector3::new(0., -90f32.to_radians() as GLfloat, 0.));
                left_quad.translate(Vector3::new(pos.0 as GLfloat - 0.5, pos.1 as GLfloat, 0.));
                left_quad.cycle_uvs(2);
                builder = builder.add_quad(left_quad);
            }
        }
        
    }
    Ok(builder.finish()?)
}


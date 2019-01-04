use std::collections::HashMap;

use gl::types::GLfloat;
use glm::Vector3;

use application::ApplicationError;
use graphics::{ Mesh, MeshBuilder, ShaderProgram, GraphicsError };
use graphics::mesh::{ Quad };
use world::{ Camera, Object, traits::{ Renderable, Translatable } };
use super::Field;

pub struct Layer {
    object: Object,
    fields: HashMap<(u32, u32), Field>,
}

impl Layer {
    pub fn new(level: i32, size: (u32, u32)) -> Result<Self, ApplicationError> {
        let mut fields = HashMap::new();
        for y in 0..size.1 {
            for x in 0..size.0 {
                fields.insert((x, y), Field::default());
            }
        }
        let mesh = create_mesh(&fields)?;
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
        let mut quad = Quad::default();
        //quad.rotate(Vector3::new(45f32.to_radians() as GLfloat, 0., 0.)); 
        quad.translate(Vector3::new(pos.0 as GLfloat, pos.1 as GLfloat, 0.));
        builder = builder.add_quad(quad);
    }
    Ok(builder.finish()?)
}


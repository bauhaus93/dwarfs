use glm::{ Matrix4, Vector3 };
use gl::types::GLfloat;
use num_traits::One;

use graphics::{ Mesh, ShaderProgram, GraphicsError };
use super::{ Camera, Model, Positionable, Rotatable, Renderable };

pub struct Object {
    model: Model,
    mesh: Mesh,
    mvp: Matrix4<GLfloat>,
    mvp_update: bool
}

impl Object {
    pub fn new(mesh: Mesh) -> Object {
        Object {
            model: Model::default(),
            mesh: mesh,
            mvp: Matrix4::<GLfloat>::one(),
            mvp_update: true
        }
    }
}

impl Renderable for Object {
    fn render(&mut self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        if self.mvp_update {
            self.mvp = camera.create_mvp_matrix(&self.model);
            self.mvp_update = false;
        }
        shader.set_mvp_matrix(&self.mvp)?;
        self.mesh.render()?;
        Ok(()) 
    }
}

impl Positionable for Object {
    fn set_position(&mut self, new_position: Vector3<f32>) {
        self.model.set_position(new_position);
        self.mvp_update = true;
    }
    fn get_position(&self) -> Vector3<f32> {
        self.model.get_position()
    }
}

impl Rotatable for Object {
    fn set_rotation(&mut self, new_rotation: Vector3<f32>) {
        self.model.set_rotation(new_rotation);
        self.mvp_update = true;
    }
    fn get_rotation(&self) -> Vector3<f32> {
        self.model.get_rotation()
    }
}

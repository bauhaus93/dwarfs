use std::ops::Add;
use gl::types::GLfloat;
use glm::Vector3;

pub trait Rotatable {
    fn set_rotation(&mut self, new_rotation: Vector3<GLfloat>);
    fn get_rotation(&self) -> Vector3<GLfloat>;
    fn mod_rotation(&mut self, offset: Vector3<GLfloat>) {
        let new_rotation = self.get_rotation().add(offset);
        self.set_rotation(new_rotation);
    }
} 

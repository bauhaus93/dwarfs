use std::ops::Add;
use gl::types::GLfloat;
use glm::Vector3;

pub trait Positionable {
    fn set_position(&mut self, new_position: Vector3<GLfloat>);
    fn get_position(&self) -> Vector3<GLfloat>;
    fn mod_position(&mut self, offset: Vector3<GLfloat>) {
        let new_pos = self.get_position().add(offset);
        self.set_position(new_pos);
    }
}

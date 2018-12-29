use std::ops::Add;
use glm::Vector3;

pub trait Positionable {
    fn set_position(&mut self, new_position: Vector3<f32>);
    fn get_position(&self) -> Vector3<f32>;
    fn mod_position(&mut self, offset: Vector3<f32>) {
        let new_pos = self.get_position().add(offset);
        self.set_position(new_pos);
    }
}

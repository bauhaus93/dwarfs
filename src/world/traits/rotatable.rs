use std::ops::Add;
use glm::Vector3;

pub trait Rotatable {
    fn set_rotation(&mut self, new_rotation: Vector3<f32>);
    fn get_rotation(&self) -> Vector3<f32>;
    fn mod_rotation(&mut self, offset: Vector3<f32>) {
        let new_rotation = self.get_rotation().add(offset);
        self.set_rotation(new_rotation);
    }
} 

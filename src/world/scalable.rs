use std::ops::Add;
use glm::Vector3;

pub trait Scalable {
    fn set_scale(&mut self, new_scale: Vector3<f32>);
    fn get_scale(&self) -> Vector3<f32>;
    fn mod_scale(&mut self, offset: Vector3<f32>) {
        let new_scale = self.get_scale().add(offset);
        self.set_scale(new_scale);
    }
}

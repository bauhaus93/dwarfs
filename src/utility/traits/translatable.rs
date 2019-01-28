use std::ops::Add;
use glm::Vector3;

pub trait Translatable {
    fn set_translation(&mut self, new_translation: Vector3<f32>);
    fn get_translation(&self) -> Vector3<f32>;
    fn mod_translation(&mut self, offset: Vector3<f32>) {
        let new_translation = self.get_translation().add(offset);
        self.set_translation(new_translation);
    }
}

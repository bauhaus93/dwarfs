
use std::fmt;
use gl::types::GLfloat;
use glm;
use glm::GenNum;
use num_traits::One;
use glm::{ Vector3, Matrix4 };

use graphics::{ create_translation_matrix, create_rotation_matrix, create_scale_matrix };
use world::traits::{ Translatable, Rotatable, Scalable };

pub struct Model {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    matrix: glm::Matrix4<GLfloat>
}

impl Model {
    fn update_matrix(&mut self) {
        self.matrix = create_translation_matrix(self.position.clone()) *
                      create_rotation_matrix(self.rotation.clone()) *
                      create_scale_matrix(self.scale.clone());
    }
    pub fn get_matrix(&self) -> Matrix4<GLfloat> {
        self.matrix.clone()
    }
}

impl Default for Model {
    fn default() -> Self {
        let mut model = Self {
            position: Vector3::<f32>::from_s(0.),
            rotation: Vector3::<f32>::from_s(0.),
            scale: Vector3::<f32>::from_s(1.),
            matrix: Matrix4::<GLfloat>::one()
        };
        model.update_matrix();
        model
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos = {:.2}/{:.2}/{:.2}, rot = {:.2}/{:.2}/{:.2}",
            self.position.x, self.position.y, self.position.z,
            self.rotation.x, self.rotation.y, self.rotation.z)
    }
}

impl Translatable for Model {
     fn set_position(&mut self, new_position: Vector3<f32>) {
        self.position = new_position;
        self.update_matrix();
    }
    fn get_position(&self) -> Vector3<f32> {
        self.position.clone()
    }
}

impl Rotatable for Model {
    fn set_rotation(&mut self, new_rotation: Vector3<f32>) {
        const DOUBLE_PI: f32 = std::f32::consts::PI * 2.;
        self.rotation = new_rotation;
        for value in self.rotation.as_array_mut().iter_mut() {
            if *value >= DOUBLE_PI {
                *value -= DOUBLE_PI;
            } else if *value < 0. {
                *value += DOUBLE_PI;
            }
        }
        self.update_matrix();
    }
    fn get_rotation(&self) -> Vector3<f32> {
        self.rotation.clone()
    }
}

impl Scalable for Model {
     fn set_scale(&mut self, new_scale: Vector3<f32>) {
        const MIN_SCALE: f32 = 1e-3;
        self.scale = new_scale;
        for value in self.scale.as_array_mut().iter_mut() {
            if *value < MIN_SCALE {
                *value = MIN_SCALE;
            }
        }
        self.update_matrix();
    }
    fn get_scale(&self) -> Vector3<f32> {
        self.scale.clone()
    }
}

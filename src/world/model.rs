use glm;
use glm::GenNum;
use num_traits::One;
use glm::{ Vector3, Matrix4 };
use gl::types::GLfloat;

use super::matrix::{ create_translation_matrix, create_rotation_matrix };
use super::{ Positionable, Rotatable };

pub struct Model {
    position: Vector3<GLfloat>,
    rotation: Vector3<GLfloat>,
    matrix: glm::Matrix4<GLfloat>
}

impl Model {
    fn update_matrix(&mut self) {
        self.matrix = create_translation_matrix(&self.position) *
                      create_rotation_matrix(&self.rotation);
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            position: Vector3::<GLfloat>::from_s(0.),
            rotation: Vector3::<GLfloat>::from_s(0.),
            matrix: Matrix4::<GLfloat>::one()
        }
    }
}

impl Positionable for Model {
     fn set_position(&mut self, new_position: Vector3<GLfloat>) {
        self.position = new_position;
        self.update_matrix();
    }
    fn get_position(&self) -> Vector3<GLfloat> {
        self.position.clone()
    }
}

impl Rotatable for Model {
    fn set_rotation(&mut self, new_rotation: Vector3<GLfloat>) {
        self.rotation = new_rotation;
        self.update_matrix();
    }
    fn get_rotation(&self) -> Vector3<GLfloat> {
        self.rotation.clone()
    }
}

use glm;
use glm::GenNum;
use num_traits::One;
use glm::{ Vector3, Matrix4 };
use gl::types::GLfloat;

use super::matrix::{ create_translation_matrix, create_rotation_matrix };

pub struct Model {
    position: Vector3<GLfloat>,
    rotation: Vector3<GLfloat>,
    matrix: glm::Matrix4<GLfloat>
}

impl Model {
    pub fn set_position(&mut self, new_position: &[GLfloat; 3]) {
        self.position = *Vector3::from_array(new_position);
        self.update_matrix();
    }
    pub fn set_rotation(&mut self, new_rotation: &[GLfloat; 3]) {
        self.rotation =*Vector3::from_array(new_rotation);
        self.update_matrix();
    }

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

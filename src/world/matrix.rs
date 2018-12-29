use glm;
use glm::{ Vector3, Matrix4 };
use gl::types::GLfloat;
use num_traits::One;

pub fn create_translation_matrix(translation: &Vector3<GLfloat>) -> glm::Matrix4<GLfloat> {
    glm::ext::translate(&glm::Matrix4::<GLfloat>::one(), *translation)
}

pub fn create_rotation_matrix(rotation: &Vector3<GLfloat>) -> glm::Matrix4<GLfloat> {
    let one = glm::Matrix4::<GLfloat>::one();
    glm::ext::rotate(&one, rotation.x, glm::Vector3::new(1., 0., 0.)) *
    glm::ext::rotate(&one, rotation.y, glm::Vector3::new(0., 1., 0.)) *
    glm::ext::rotate(&one, rotation.z, glm::Vector3::new(0., 0., 1.))
}



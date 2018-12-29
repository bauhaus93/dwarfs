use glm;
use glm::{ Vector3, Matrix4, normalize };
use gl::types::GLfloat;
use num_traits::One;

pub fn create_translation_matrix(translation: Vector3<f32>) -> glm::Matrix4<GLfloat> {
    glm::ext::translate(&glm::Matrix4::<GLfloat>::one(), translation)
}

pub fn create_rotation_matrix(rotation: Vector3<f32>) -> glm::Matrix4<GLfloat> {
    let one = glm::Matrix4::<GLfloat>::one();
    glm::ext::rotate(&one, rotation.x as GLfloat, glm::Vector3::<GLfloat>::new(1., 0., 0.)) *
    glm::ext::rotate(&one, rotation.y as GLfloat, glm::Vector3::<GLfloat>::new(0., 1., 0.)) *
    glm::ext::rotate(&one, rotation.z as GLfloat, glm::Vector3::<GLfloat>::new(0., 0., 1.))
}

pub fn create_direction(rotation: Vector3<f32>) -> Vector3<f32> {
    normalize(Vector3::<f32>::new(
        rotation.y.sin() * rotation.x.cos(),
        rotation.y.sin() * rotation.x.sin(),
        rotation.y.cos()))
}

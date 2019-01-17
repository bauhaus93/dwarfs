use glm;
use glm::{ Vector3, Vector4, Matrix4, normalize };
use gl::types::GLfloat;
use num_traits::One;

pub fn create_transformation_matrix(translation: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Matrix4<GLfloat> {
    create_translation_matrix(translation) * create_rotation_matrix(rotation) * create_scale_matrix(scale) 
}

pub fn create_translation_matrix(translation: Vector3<f32>) -> Matrix4<GLfloat> {
    glm::ext::translate(&Matrix4::<GLfloat>::one(), translation)
}

pub fn create_rotation_matrix(rotation: Vector3<f32>) -> Matrix4<GLfloat> {
    let one = Matrix4::<GLfloat>::one();
    glm::ext::rotate(&one, rotation.x as GLfloat, glm::Vector3::<GLfloat>::new(1., 0., 0.)) *
    glm::ext::rotate(&one, rotation.y as GLfloat, glm::Vector3::<GLfloat>::new(0., 1., 0.)) *
    glm::ext::rotate(&one, rotation.z as GLfloat, glm::Vector3::<GLfloat>::new(0., 0., 1.))
}

pub fn create_scale_matrix(scale: Vector3<f32>) -> Matrix4<GLfloat> {
    glm::ext::scale(&Matrix4::<GLfloat>::one(), scale)
}

pub fn create_direction(rotation: Vector3<f32>) -> Vector3<f32> {
    normalize(Vector3::<f32>::new(
        rotation.y.sin() * rotation.x.cos(),
        rotation.y.sin() * rotation.x.sin(),
        rotation.y.cos()))
}

pub fn create_orthographic_projection_matrix(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Matrix4<GLfloat> {
    let trans_input = Matrix4::<GLfloat>::new(Vector4::<GLfloat>::new(1., 0., 0., 0.), Vector4::<GLfloat>::new(0., 1., 0., 0.),
                                              Vector4::<GLfloat>::new(0., 0., -1., 0.), Vector4::<GLfloat>::new(0., 0., 0., 1.));
    create_scale_matrix(Vector3::<GLfloat>::new(2. / (right - left), 2. / (top - bottom), 2. / (far - near))) * 
    glm::ext::translate(&trans_input, Vector3::<GLfloat>::new(-(left + right) / 2., -(top + bottom) / 2., -(far + near) / 2.))
}

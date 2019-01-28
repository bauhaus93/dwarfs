use gl::types::{ GLfloat };
use glm::{ Vector3, Matrix4, GenNum };

use utility::traits::{ Translatable, Rotatable, Scalable, Transformable };
use graphics::transformation::create_transformation_matrix;
use super::Triangle;

pub struct Node {
    translation: Vector3<GLfloat>,
    rotation: Vector3<GLfloat>,
    scale: Vector3<GLfloat>,
    triangles: Vec<Triangle>,
}

impl Node {

    pub fn create_transformed_triangles(&self) -> Vec<Triangle> {
        let transformation_matrix = create_transformation_matrix(self.translation, self.rotation, self.scale);
        let mut transformed_triangles = self.triangles.clone();
        transformed_triangles.iter_mut()
            .for_each(|t| t.transform(transformation_matrix));
        transformed_triangles
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            translation: Vector3::from_s(0.),
            rotation: Vector3::from_s(0.),
            scale: Vector3::from_s(1.),
            triangles: Vec::new()
        }
    }
}

impl Translatable for Node {
    fn set_translation(&mut self, new_translation: Vector3<GLfloat>) {
        self.translation = new_translation;
    }
    fn get_translation(&self) -> Vector3<GLfloat> {
        self.translation
    }
}

impl Rotatable for Node {
    fn set_rotation(&mut self, new_rotation: Vector3<GLfloat>) {
        self.translation = new_rotation;
    }
    fn get_rotation(&self) -> Vector3<GLfloat> {
        self.rotation
    }
}

impl Scalable for Node {
    fn set_scale(&mut self, new_scale: Vector3<GLfloat>) {
        self.scale = new_scale;
    }
    fn get_scale(&self) -> Vector3<GLfloat> {
        self.scale
    }
}
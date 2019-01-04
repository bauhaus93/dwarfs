use std::ops::Add;
use gl::types::GLfloat;
use glm::{ GenNum, Vector3, Vector4 };

use graphics::{ create_rotation_matrix };

#[derive(Clone, Copy)]
pub struct Vertex {
    pos: Vector3<GLfloat>,
    uv: Vector3<GLfloat>,
    normal: Vector3<GLfloat>
}

impl Vertex {

    pub fn get_pos(&self) -> Vector3<GLfloat> {
        self.pos.clone()
    }
    pub fn get_uv(&self) -> Vector3<GLfloat> {
        self.uv.clone()
    }

    pub fn get_normal(&self) -> Vector3<GLfloat> {
        self.normal.clone()
    }

    pub fn set_pos(&mut self, new_pos: Vector3<GLfloat>) {
        self.pos = new_pos;
    }
    pub fn set_uv(&mut self, new_uv: Vector3<GLfloat>) {
        self.uv = new_uv;
    }
    pub fn set_normal(&mut self, new_normal: Vector3<GLfloat>) {
        self.normal = new_normal;
    }

    pub fn translate(&mut self, translation: Vector3<GLfloat>) {
        self.pos = self.pos.add(translation);
    }

    pub fn rotate(&mut self, rotation: Vector3<GLfloat>) {
        let rot_mat = create_rotation_matrix(rotation);
        let new_pos = rot_mat * Vector4::new(self.pos.x, self.pos.y, self.pos.z, 1.0);
        let new_normal = rot_mat * Vector4::new(self.normal.x, self.normal.y, self.normal.z, 1.0);
        for i in 0..3 {
            self.pos[i] = new_pos[i];
            self.normal[i] = new_normal[i];
        }
    }

}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            pos: Vector3::from_s(0.),
            uv: Vector3::from_s(0.),
            normal: Vector3::from_s(0.), 
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        const THRESHOLD: GLfloat = 1e-3;
        self.pos.as_array().iter().chain(self.uv.as_array().iter()).chain(self.normal.as_array().iter())
            .zip(other.pos.as_array().iter().chain(other.uv.as_array().iter()).chain(other.normal.as_array().iter()))
            .all(|(&a, &b)| (a - b).abs() < THRESHOLD)
    }
}

impl Eq for Vertex {}



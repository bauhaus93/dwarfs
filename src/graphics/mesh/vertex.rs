use std::ops::Add;
use std::cmp::Ordering;
use std::fmt;
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

    pub fn set_uv_layer(&mut self, layer: u32) {
        self.uv.z = layer as GLfloat;
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

fn cmp_vec(lhs: &Vector3<GLfloat>, rhs: &Vector3<GLfloat>) -> Ordering {
    const THRESHOLD: GLfloat = 1e-3;
    for i in 0..3 {
        let diff = lhs[i] - rhs[i];
        if diff < -THRESHOLD {
            return Ordering::Less;
        } else if diff > THRESHOLD {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        match cmp_vec(&self.pos, &other.pos) {
            Ordering::Equal => {
                match cmp_vec(&self.uv, &other.uv) {
                    Ordering::Equal => {
                        match cmp_vec(&self.normal, &other.normal) {
                            Ordering::Equal => true,
                            _ => false
                        }
                    },
                    _ => false
                }
            },
            _ => false
        }
    }
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        match cmp_vec(&self.pos, &other.pos) {
            Ordering::Equal => {
                match cmp_vec(&self.uv, &other.uv) {
                    Ordering::Equal => {
                        match cmp_vec(&self.normal, &other.normal) {
                            Ordering::Equal => Ordering::Equal,
                            order => order
                        }
                    },
                    order => order
                }
            },
            order => order
        }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v = {:.2}/{:.2}/{:.2}, uv = {:.2}/{:.2}/{:.2}, n = {:.2}/{:.2}/{:.2}",
            self.pos[0], self.pos[1], self.pos[2],
            self.uv[0], self.uv[1], self.uv[2],
            self.normal[0], self.normal[1], self.normal[2])
    }
}






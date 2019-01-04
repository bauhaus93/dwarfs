use gl::types::GLfloat;
use glm::{ Vector3, Vector4 };

use super::{ Vertex, Triangle };

const POS_OFFSET: [[GLfloat; 3]; 4] = [
    [-0.5, -0.5, 0.],
    [0.5, -0.5, 0.],
    [0.5, 0.5, 0.],
    [-0.5, 0.5, 0.]
];

const UVS: [[GLfloat; 3]; 4] = [
    [0., 0., 0.],
    [1., 0., 0.],
    [1., 1., 0.],
    [0., 1., 0.]
];

pub struct Quad {
    vertices: [Vertex; 4]
}

impl Quad {

    pub fn set_vertex(&mut self, index: usize, vertex: Vertex) {
        debug_assert!(index < 4);
        self.vertices[index] = vertex;
    }

    pub fn translate(&mut self, translation: Vector3<GLfloat>) {
        for v in self.vertices.iter_mut() {
            v.translate(translation);
        }
    }

    pub fn rotate(&mut self, rotation: Vector3<GLfloat>) {
        for v in self.vertices.iter_mut() {
            v.rotate(rotation);
        }
    }
    
    pub fn cycle_uvs(&mut self, cycles: u8) {
        let mut new_uvs: Vec<Vector3<GLfloat>> = Vec::new();
        for v in self.vertices.iter().cycle().skip(cycles as usize).take(4) {
            new_uvs.push(v.get_uv());
        }
        for (vert, uv) in self.vertices.iter_mut().zip(new_uvs.into_iter()) {
            vert.set_uv(uv)
        }
    }

    pub fn set_uv_layer(&mut self, layer: u32) {
        for v in self.vertices.iter_mut() {
            v.set_uv_layer(layer);
        }
    }
    
    pub fn to_triangles(&self) -> [Triangle; 2] {
        [Triangle::new([self.vertices[0].clone(), self.vertices[1].clone(), self.vertices[2].clone()]),
         Triangle::new([self.vertices[2].clone(), self.vertices[3].clone(), self.vertices[0].clone()])]
    }
}

impl Default for Quad {
    fn default() -> Self {
        let mut vertices = [Vertex::default(); 4];
        for (index, (vert, uv)) in POS_OFFSET.iter().zip(UVS.iter()).enumerate() {
            vertices[index].translate(Vector3::new(vert[0], vert[1], vert[2]));
            vertices[index].set_uv(Vector3::new(uv[0], uv[1], uv[2]));
            vertices[index].set_normal(Vector3::new(0., 0., 1.));
        }
        Self {
            vertices: vertices
        }
    }
}


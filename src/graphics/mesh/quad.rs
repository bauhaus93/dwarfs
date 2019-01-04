use gl::types::GLfloat;

use super::{ Vertex, Triangle };

const VERTICES: [[GLfloat; 3]; 4] = [
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
const NORMAL: [GLfloat; 3] = [0., 0., 1.];

pub struct Quad {
    vertices: [Vertex; 4]
}

impl Quad {
    pub fn rotated_90(&mut self, axis: u8) {
        let mut quad = Quad::default();
        let iter_vert = VERTICES.iter().skip(2 - axis).cycle.take(4);
        for (index, (vert, uv)) in iter_vert.zip(UVS.iter()).enumerate() {
            let vertex = Vertex::new(*vert, *uv, NORMAL);
            quad.set_vertex(index, vertex);
        }
        quad
    }

    pub fn set_vertex(&mut self, index: usize, vertex: Vertex) {
        debug_assert!(index < 4);
        self.vertices[index] = vertex;
    }

    
    pub fn to_triangles(&self) -> [Triangle; 2] {
        [Triangle::new([self.vertices[0].clone(), self.vertices[1].clone(), self.vertices[2].clone()]),
         Triangle::new([self.vertices[2].clone(), self.vertices[3].clone(), self.vertices[0].clone()])]
    }
}

impl Default for Quad {
    fn default() -> Self {
        let mut quad = Self {
            vertices: [Vertex::default(); 4]
        };
        for (index, (vert, uv)) in VERTICES.iter().zip(UVS.iter()).enumerate() {
            let vertex = Vertex::new(*vert, *uv, NORMAL);
            quad.set_vertex(index, vertex);
        }
        quad
    }
}


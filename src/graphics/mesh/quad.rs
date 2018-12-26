use super::{ Vertex, Triangle };

pub struct Quad {
    vertices: [Vertex; 4]
}

impl Quad {
    pub fn new(vertices: [Vertex; 4]) -> Quad {
        Quad {
            vertices: vertices
        }
    }

    pub fn to_triangles(&self) -> [Triangle; 2] {
        [Triangle::new([self.vertices[0].clone(), self.vertices[1].clone(), self.vertices[2].clone()]),
         Triangle::new([self.vertices[2].clone(), self.vertices[3].clone(), self.vertices[0].clone()])]
    }
}


use super::Vertex;

pub struct Triangle {
  vertices: [Vertex; 3]
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Triangle {
        Triangle {
            vertices: vertices
        }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    
    pub fn into_vertices(self) -> [Vertex; 3] {
        self.vertices
    }
}

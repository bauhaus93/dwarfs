use std::fmt;

use super::Vertex;

pub struct Triangle {
  vertex: [Vertex; 3]
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Triangle {
        Triangle {
            vertex: vertices
        }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertex
    }

    pub fn set_vertex(&mut self, vertex: Vertex, index: usize) {
        debug_assert!(index < 3);
        self.vertex[index] = vertex;
    }
    
    pub fn into_vertices(self) -> [Vertex; 3] {
        self.vertex
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertex: [Vertex::default(); 3]
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vertices: {}, {}, {}", self.vertex[0], self.vertex[1], self.vertex[2])
    }
}

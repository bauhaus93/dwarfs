use gl::types::{ GLfloat, GLuint };

use super::{ Vertex, Triangle, Quad, Mesh };

pub struct MeshBuilder { 
    triangles: Vec<Triangle>,
    indexed_vertices: Vec<(Vertex, GLuint)>,
    position_buffer: Vec<GLfloat>,
    uv_buffer: Vec<GLfloat>,
    normal_buffer: Vec<GLfloat>,
    index_buffer: Vec<GLuint>,
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            triangles: Vec::new(),
            indexed_vertices: Vec::new(),
            position_buffer: Vec::new(),
            uv_buffer: Vec::new(),
            normal_buffer: Vec::new(),
            index_buffer: Vec::new()
        }
    }

    pub fn add_quad(self, quad: Quad) -> Self {
        quad.to_triangles().into_iter().fold(self, |s, t| s.add_triangle(t))
    }

    pub fn add_triangle(mut self, triangle: &Triangle) -> Self {
        for vert in triangle.get_vertices().iter() {
            let new_vert_index = match self.indexed_vertices.iter().find(|(v, _i)| v == vert) {
                Some((_, i)) => {
                    self.index_buffer.push(*i);
                    None
                },
                None => {
                    debug_assert!(self.position_buffer.len() % 3 == 0);
                    debug_assert!(self.uv_buffer.len() % 3 == 0);
                    debug_assert!(self.normal_buffer.len() % 3 == 0);
                    let new_index = (self.position_buffer.len() / 3) as GLuint;
                    self.position_buffer.extend(vert.get_pos());
                    self.uv_buffer.extend(vert.get_uv());
                    self.normal_buffer.extend(vert.get_normal());
                    self.index_buffer.push(new_index);
                    Some((vert.clone(), new_index))
                }
            };
            match new_vert_index {
                Some(vi) => self.indexed_vertices.push(vi),
                None => {}
            }
        }
        self 
    }


}


use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::{ ptr, io, ffi::c_void, mem::size_of };
use gl;
use gl::types::{ GLfloat, GLint, GLuint, GLenum, GLsizeiptr };
use glm::Matrix4;

use utility::read_obj;
use graphics::{ check_opengl_error, OpenglError, mesh::{ Vertex, Triangle } };
use super::{ VAO, Node, MeshError };

pub struct Mesh {
    vao: VAO,
    nodes: Vec<Node>
}


impl Mesh {
    pub fn from_obj(obj_path: &str) -> Result<Mesh, MeshError> {
        Self::from_triangles(&read_obj(obj_path)?)
    }

    pub fn add_node(&mut self, node: Node){
        self.nodes.push(node);
    }
    pub fn get_vertex_count(&self) -> u32 {
        self.vao.get_index_count()
    }

    pub fn from_triangles(triangles: &[Triangle]) -> Result<Mesh, MeshError> {
        let mut node = Node::default();
        triangles.iter()
            .for_each(|t| node.add_triangle(*t));
        let vao = VAO::new(&node.create_transformed_triangles())?;
        let mut nodes = Vec::new();
        nodes.push(node);
        Ok(
            Self {
                vao: vao,
                nodes: nodes
            }
        )
    }

    pub fn render(&self) -> Result<(), MeshError> {
        self.vao.render()
    }
}

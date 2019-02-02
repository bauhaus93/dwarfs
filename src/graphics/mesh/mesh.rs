use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::{ ptr, io, ffi::c_void, mem::size_of, ops::Sub };
use gl;
use gl::types::{ GLfloat, GLint, GLuint, GLenum, GLsizeiptr };
use glm::{ Matrix4, Vector3, builtin::{ dot, normalize } };

use utility::read_obj;
use graphics::{ check_opengl_error, OpenglError, mesh::{ Vertex, Triangle } };
use super::{ VAO, Node, MeshError, BuildOption };

pub struct Mesh {
    vao: Option<VAO>,
    nodes: Vec<Node>
}

impl Mesh {
    pub fn from_obj(obj_path: &str) -> Result<Mesh, MeshError> {
        Self::from_triangles(&read_obj(obj_path)?)
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
                vao: Some(vao),
                nodes: nodes
            }
        )
    }

    fn build_triangles(&mut self, triangles: &[Triangle]) -> Result<(), MeshError> {
        if triangles.len() > 0 {
            self.vao = Some(VAO::new(triangles)?);
        }
        Ok(())
    }

    pub fn build(&mut self) -> Result<(), MeshError> {
        let triangles = self.copy_triangles();
        self.build_triangles(&triangles)
    }

    pub fn build_without_invisible(&mut self, camera_direction: Vector3<GLfloat>) -> Result<(), MeshError> {
        let mut triangles = self.copy_triangles();
        remove_incident_triangles(&mut triangles);
        remove_triangles_by_direction(&mut triangles, camera_direction);
        self.build_triangles(&triangles)
    }

    pub fn add_node(&mut self, node: Node){
        self.nodes.push(node);
    }
    pub fn get_vertex_count(&self) -> u32 {
        match self.vao {
            Some(ref vao) => vao.get_index_count(),
            _ => 0
        }
    }

    pub fn copy_triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        for node in self.nodes.iter() {
            triangles.extend(node.create_transformed_triangles());
        }
        triangles
    }

    pub fn render(&self) -> Result<(), MeshError> {
        match self.vao {
            Some(ref vao) => vao.render(),
            None => { Ok(()) }
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vao: None,
            nodes: Vec::new()
        }
    }
}

fn remove_incident_triangles(triangles: &mut Vec<Triangle>) {

}

fn remove_triangles_by_direction(triangles: &mut Vec<Triangle>, dir_vec: Vector3<f32>) {
    triangles.retain(|t| dot(t.get_normal(), dir_vec) <= 0.);
}

use std::collections::{ HashMap, BTreeSet };
use std::cmp::Ordering;
use std::ops::{ Sub };

use gl::types::GLfloat;
use glm::{ Vector3, builtin::{ cross, normalize } };

use utility::{ cmp_vec, traits::Translatable };
use graphics::{  GraphicsError, mesh::{ MeshError, Node, Mesh, MeshManager, Triangle } };
use world::WorldError;
use super::Field;

struct TriangleEntry {
    triangle: Triangle,
    visible: bool
}

pub fn create_mesh(fields: &HashMap<(i32, i32), Field>, mesh_manager: &MeshManager) -> Result<Mesh, MeshError> {
    let mut mesh = Mesh::default();
    for (pos, field) in fields {
        let translation: Vector3<f32> = Vector3::new(pos.0 as f32, pos.1 as f32, 0.);
        let mut node = create_cube_node(mesh_manager)?;
        node.set_translation(translation);
        mesh.add_node(node);
    }
    mesh.build_vao()?;
    Ok(mesh)
}

fn create_cube_node(mesh_manager: &MeshManager) -> Result<Node, MeshError> {
    let mut node = Node::default();
    let triangles = mesh_manager.get_mesh("cube")?.copy_triangles();
    node.add_triangles(triangles);
    Ok(node)
}

//assumes points ordered ccw
fn calculate_normal(points: &[Vector3<f32>; 3]) -> Vector3<f32> {
    let a = points[0].sub(points[1]);
    let b = points[1].sub(points[2]);
    normalize(cross(a, b))
}

impl TriangleEntry {
    pub fn new(triangle: Triangle) -> Self {
        Self {
            triangle: triangle,
            visible: true
        }
    }
    pub fn set_invisible(&mut self) {
        self.visible = false;
    }
    pub fn into_triangle(self) -> Option<Triangle> {
        if self.visible {
            Some(self.triangle)
        } else {
            None
        }
    }
}

impl PartialEq for TriangleEntry {
    fn eq(&self, other: &Self) -> bool {
        self.triangle.as_vertices().iter()
            .zip(other.triangle.as_vertices().iter())
            .all(|(lhs, rhs)| cmp_vec(&lhs.get_pos(), &rhs.get_pos()) == Ordering::Equal) ||
        self.triangle.as_vertices().iter().cycle().skip(1).take(3)
            .zip(other.triangle.as_vertices().iter())
            .all(|(lhs, rhs)| cmp_vec(&lhs.get_pos(), &rhs.get_pos()) == Ordering::Equal) ||
        self.triangle.as_vertices().iter().cycle().skip(2).take(3)
            .zip(other.triangle.as_vertices().iter())
            .all(|(lhs, rhs)| cmp_vec(&lhs.get_pos(), &rhs.get_pos()) == Ordering::Equal)
    }
}

impl Eq for TriangleEntry {}

impl Ord for TriangleEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let iter = self.triangle.as_vertices().iter()
            .zip(other.triangle.as_vertices().iter());
        for (lhs, rhs) in iter {
            let result = cmp_vec(&lhs.get_pos(), &rhs.get_pos());
            if result != Ordering::Equal {
                return result;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for TriangleEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


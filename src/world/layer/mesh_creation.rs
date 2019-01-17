use std::collections::{ HashMap, BTreeSet };
use std::cmp::Ordering;

use gl::types::GLfloat;
use glm::Vector3;

use utility::cmp_vec;
use graphics::{ Mesh, MeshManager, Triangle, GraphicsError };
use world::WorldError;
use super::Field;

struct TriangleEntry {
    triangle: Triangle,
    visible: bool
}

pub fn create_mesh(fields: &HashMap<(i32, i32), Field>, mesh_manager: &MeshManager, upper_fields: Option<&HashMap<(i32, i32), Field>>) -> Result<Mesh, WorldError> {
    let mut triangles: BTreeSet<TriangleEntry> = BTreeSet::new();

    Err(WorldError::from(GraphicsError::FunctionFailure("Not implemented".to_string())))
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


use std::collections::{ HashMap, BTreeSet };
use std::cmp::Ordering;

use gl::types::GLfloat;
use glm::Vector3;

use utility::cmp_vec;
use graphics::{ Mesh, MeshManager, Triangle, GraphicsError, mesh::MeshError };
use world::WorldError;
use super::Field;

struct TriangleEntry {
    triangle: Triangle,
    visible: bool
}

pub fn create_mesh(fields: &HashMap<(i32, i32), Field>, mesh_manager: &MeshManager, upper_fields: Option<&HashMap<(i32, i32), Field>>) -> Result<Mesh, MeshError> {
    /*let mut triangle_entries: BTreeSet<TriangleEntry> = BTreeSet::new();

    for (pos, field) in fields {
        let mut field_triangles = match mesh_manager.get_mesh("block") {
            Some(mesh) => mesh.copy_triangles(),
            None => return Err(MeshError::MeshNotFound("block".to_string()))
        };
        for mut t in field_triangles.into_iter() {
            t.translate_vertices(Vector3::new(2. * pos.0 as f32, 2. * pos.1 as f32, 0.));
            let mut entry = TriangleEntry::new(t);
            if triangle_entries.contains(&entry) {
                entry.set_invisible();
                triangle_entries.replace(entry);
                debug!("Made invisible!");
            } else {
                triangle_entries.insert(entry);
            }
        }
    }
    let mut triangles: Vec<Triangle> = Vec::with_capacity(triangle_entries.len());
    for entry in triangle_entries.into_iter() {
        match entry.into_triangle() {
            Some(t) => triangles.push(t),
            _ => {}
        }
    }
    let mesh = Mesh::from_triangles(triangles)?;
    Ok(mesh)*/
    Err(MeshError::MeshNotFound("LEL".to_string()))
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


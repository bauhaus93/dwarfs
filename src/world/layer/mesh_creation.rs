use std::collections::{ HashMap, BTreeSet };
use std::convert::TryFrom;
use std::cmp::Ordering;
use std::ops::{ Sub };
use std::time;

use glm::{ Vector3, GenNum, builtin::{ dot, normalize } };

use utility::{ Float, cmp_vec, traits::{ Translatable, Rotatable, Scalable } };
use graphics::{  GraphicsError, mesh::{ Buffer, Vertex, VAO, MeshError, Node, Mesh, MeshManager, Triangle } };
use world::{ WorldError, Direction, DIRECTION_VECTOR };
use super::{ Field, FieldType, FieldMaterial };

pub fn create_mesh(fields: &HashMap<[i32; 2], Field>, mesh_manager: &MeshManager, camera_direction: Vector3<Float>) -> Result<Mesh, MeshError> {
    let start_time = time::Instant::now();

    let mut mesh = Mesh::default();
    for (pos, field) in fields {
        let mut node = Node::default();

        node.set_translation(Vector3::new(pos[0] as Float, pos[1] as Float, 0.));

        let triangles = match field.get_type() {
            FieldType::CUBE => mesh_manager.get_mesh("cube")?.copy_triangles(),
            FieldType::SLOPE(dir) => {
                match dir {
                    Direction::North => { node.set_rotation(Vector3::new(0., 0., 180f32.to_radians())) },
                    Direction::East => { node.set_rotation(Vector3::new(0., 0., 270f32.to_radians())) },
                    Direction::South => { },
                    Direction::West => { node.set_rotation(Vector3::new(0., 0., 90f32.to_radians()))},
                    _ => { warn!("Slope with invalid direction supplied to mesh_create"); }
                }
                mesh_manager.get_mesh("slope")?.copy_triangles()
            }
        };

        node.add_triangles(triangles);
        mesh.add_node(node);
    }

    let triangles = mesh.copy_triangles();
    trace!("Unfilteded triangle count = {}", triangles.len());
    let filtered_triangles = remove_incident_triangles(triangles);
    trace!("After incident removal = {}", filtered_triangles.len());
    let filtered_triangles = remove_triangles_by_direction(filtered_triangles, camera_direction);
    trace!("After directional removal = {}", filtered_triangles.len());
    let buffer = Buffer::from(filtered_triangles);
    if !buffer.is_empty() {
        let vao = VAO::try_from(buffer)?;
        mesh.set_vao(vao);
    }

    let creation_time = start_time.elapsed().as_secs() as u32 * 1000 + start_time.elapsed().subsec_millis();
    debug!("Layer mesh stats: vertices = {}, creation time = {}ms", mesh.get_vertex_count(), creation_time);

    Ok(mesh)
}


fn remove_incident_triangles(triangles: Vec<Triangle>) -> Vec<Triangle> {
    let mut triangle_set: BTreeSet<TriangleEntry> = BTreeSet::new();
    for t in triangles.into_iter() {
        let new_entry = TriangleEntry::new(t);
        let updated_entry = match triangle_set.take(&new_entry) {
            Some(mut existing_entry) => {
                existing_entry.set_invisible();
                existing_entry
            },
            None => new_entry
        };
        triangle_set.insert(updated_entry);
    }

    let mut visible_triangles = Vec::new();
    for entry in triangle_set {
        if let Some(t) = entry.into_triangle() {
            visible_triangles.push(t);
        }
    }
    visible_triangles
}

fn remove_triangles_by_direction(triangles: Vec<Triangle>, dir_vec: Vector3<Float>) -> Vec<Triangle> {
    triangles.into_iter().filter(|t| dot(t.get_normal(), dir_vec) <= 0.).collect()
}

struct TriangleEntry {
    triangle: Triangle,
    sorted_vertices: [Vertex; 3],
    visible: bool
}

impl TriangleEntry {
    pub fn new(triangle: Triangle) -> Self {
        let sorted_vertices = triangle.get_sorted_vertices();
        Self {
            triangle: triangle,
            sorted_vertices: sorted_vertices,
            visible: true
        }
    }
    pub fn set_invisible(&mut self) {
        self.visible = false;
    }
    pub fn get_triangle(&self) -> &Triangle {
        &self.triangle
    }
    pub fn into_triangle(self) -> Option<Triangle> {
        if self.visible {
            Some(self.triangle)
        } else {
            None
        }
    }
}

impl Ord for TriangleEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let iter = self.sorted_vertices.iter()
        .zip(other.sorted_vertices.iter());
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

impl PartialEq for TriangleEntry {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for TriangleEntry {}


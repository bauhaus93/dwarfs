use std::collections::{ HashMap, BTreeSet };
use std::cmp::Ordering;
use std::ops::{ Sub };

use gl::types::GLfloat;
use glm::{ Vector3, GenNum, builtin::{ cross, normalize } };

use utility::{ cmp_vec, traits::{ Translatable, Scalable } };
use graphics::{  GraphicsError, mesh::{ MeshError, Node, Mesh, MeshManager, Triangle } };
use world::{ WorldError, Direction };
use super::Field;

pub fn create_mesh(fields: &HashMap<[i32; 2], Field>, mesh_manager: &MeshManager) -> Result<Mesh, MeshError> {
    const NEIGHBOURS: [(Direction, [i32; 2]); 4] = [
        (Direction::NORTH, [0, -1]),
        (Direction::SOUTH, [0, 1]),
        (Direction::EAST, [1, 0]),
        (Direction::WEST, [-1, 0]),
    ];
    let mut mesh = Mesh::default();
    for (pos, field) in fields {
        let translation: Vector3<f32> = Vector3::new(pos[0] as f32, pos[1] as f32, 0.);
        let mut triangles = mesh_manager.get_mesh("cube")?.copy_triangles();

        for (dir, offset) in NEIGHBOURS.iter() {
            let nb_pos = [pos[0] + offset[0],
                          pos[1] + offset[1]];
            match fields.get(&nb_pos) {
                Some(_f) => {
                    remove_border_triangles(&mut triangles, *dir);
                },
                _ => {}
            }
        }
        //TODO remove triangles by normals

        let mut node = Node::default();
        node.add_triangles(triangles);
        node.set_translation(translation);
        mesh.add_node(node);
    }
    mesh.build_vao()?;
    Ok(mesh)
}

fn remove_border_triangles(triangles: &mut Vec<Triangle>, border: Direction) {
    let (axis, value): (usize, f32) = match border {
        Direction::NORTH => (1, -0.5),
        Direction::SOUTH => (1, 0.5),
        Direction::EAST => (0, 0.5),
        Direction::WEST => (0, -0.5),
        Direction::UP => (2, 0.5),
        Direction::DOWN => (2, -0.5)
    };
    triangles.retain(|t| !t.on_plane(axis, value));
}

//assumes points ordered ccw
fn calculate_normal(points: &[Vector3<f32>; 3]) -> Vector3<f32> {
    let a = points[0].sub(points[1]);
    let b = points[1].sub(points[2]);
    normalize(cross(a, b))
}
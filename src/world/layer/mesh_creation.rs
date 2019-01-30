use std::collections::{ HashMap, BTreeSet };
use std::cmp::Ordering;
use std::ops::{ Sub };

use gl::types::GLfloat;
use glm::{ Vector3, GenNum, builtin::{ cross, normalize } };

use utility::{ cmp_vec, traits::{ Translatable, Rotatable, Scalable } };
use graphics::{  GraphicsError, mesh::{ MeshError, Node, Mesh, MeshManager, Triangle } };
use world::{ WorldError, Direction };
use super::{ Field, FieldType, FieldMaterial, NEIGHBOUR_RELATION };

pub fn create_mesh(fields: &HashMap<[i32; 2], Field>, mesh_manager: &MeshManager) -> Result<Mesh, MeshError> {
    let mut mesh = Mesh::default();
    for (pos, field) in fields {
        let mut node = Node::default();

        node.set_translation(Vector3::new(pos[0] as f32, pos[1] as f32, 0.));

        let mut triangles = match field.get_type() {
            FieldType::CUBE => mesh_manager.get_mesh("cube")?.copy_triangles(),
            FieldType::SLOPE(dir) => {
                match dir {
                    Direction::NORTH => { node.set_rotation(Vector3::new(0., 0., 180f32.to_radians())) },
                    Direction::EAST => { node.set_rotation(Vector3::new(0., 0., 270f32.to_radians())) },
                    Direction::SOUTH => { },
                    Direction::WEST => { node.set_rotation(Vector3::new(0., 0., 90f32.to_radians()))},
                    _ => { warn!("Slope with invalid direction supplied to mesh_create"); }
                }
                mesh_manager.get_mesh("slope")?.copy_triangles()
            }
        };

        for (dir, offset) in NEIGHBOUR_RELATION.iter() {
            let nb_pos = [pos[0] + offset[0],
                          pos[1] + offset[1]];
            match fields.get(&nb_pos) {
                Some(_f) => {
                    //remove_border_triangles(&mut triangles, *dir);
                },
                _ => {}
            }
        }
        //TODO remove triangles by normals

        node.add_triangles(triangles);

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
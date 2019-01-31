use std::collections::{ HashMap, BTreeSet };
use std::cmp::Ordering;
use std::ops::{ Sub };

use gl::types::GLfloat;
use glm::{ Vector3, GenNum, builtin::{ cross, normalize } };

use utility::{ cmp_vec, traits::{ Translatable, Rotatable, Scalable } };
use graphics::{  GraphicsError, mesh::{ MeshError, Node, Mesh, MeshManager, Triangle, BuildOption } };
use world::{ WorldError, Direction, DIRECTION_VECTOR };
use super::{ Field, FieldType, FieldMaterial };

pub fn create_mesh(fields: &HashMap<[i32; 2], Field>, mesh_manager: &MeshManager, camera_direction: Vector3<f32>) -> Result<Mesh, MeshError> {
    let mut mesh = Mesh::default();
    for (pos, field) in fields {
        let mut node = Node::default();

        node.set_translation(Vector3::new(pos[0] as f32, pos[1] as f32, 0.));

        let mut triangles = match field.get_type() {
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
    let build_options = [BuildOption::RemoveByDirection(camera_direction)];
    mesh.build(Some(&build_options))?;
    Ok(mesh)
}

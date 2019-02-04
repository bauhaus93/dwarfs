use std::collections::{ VecDeque, BTreeSet };
use glm::Vector3;

use graphics::{ Mesh, MeshManager };
use world::{ WorldError, height_map::HeightMap };
use utility::Float;
use super::Layer;

pub struct LayerCreator {
    layer_size: [i32; 2],
    height_map: HeightMap,
    mesh_manager: MeshManager,
    camera_direction: Vector3<Float>,
    request_queue: VecDeque<i32>
}

fn load_terrain_meshes() -> Result<MeshManager, WorldError> {
    let mut mesh_manager = MeshManager::default();
    mesh_manager.add_mesh(Mesh::from_obj("resources/obj/cube.obj")?, "cube");
    mesh_manager.add_mesh(Mesh::from_obj("resources/obj/slope.obj")?, "slope");
    Ok(mesh_manager)
}

impl LayerCreator {
    pub fn new(layer_size: [i32; 2], height_map: HeightMap, camera_direction: Vector3<Float>) -> Result<Self, WorldError> {
        let creator = Self {
            layer_size: layer_size,
            height_map: height_map,
            mesh_manager: load_terrain_meshes()?,
            camera_direction: camera_direction,
            request_queue: VecDeque::new()
        };
        Ok(creator)
    }

    pub fn request_layer(&mut self, level: i32) {
        self.request_queue.push_back(level);
    }

    pub fn get_finished_layers(&mut self) -> BTreeSet<Layer> {
        let finished_layers = BTreeSet::new();

        finished_layers
    }
}

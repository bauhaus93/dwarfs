use std::rc::Rc;
use std::collections::{ BTreeMap, btree_map::Entry };

use super::{ Mesh, MeshError };

pub struct MeshManager {
    mesh_map: BTreeMap<String, Rc<Mesh>>
}

impl MeshManager {
    pub fn add_mesh(&mut self, mesh: Mesh, id: &str) {
        match self.mesh_map.entry(id.to_string()) {
            Entry::Occupied(_) => {
                warn!("Mesh with id = '{}' already existing in mesh manager", id);
            },
            Entry::Vacant(v) => {
                v.insert(Rc::new(mesh));
                trace!("Added mesh with id = '{}' to mesh manager", id);
            }
        }
    }

    pub fn get_mesh_rc(&self, id: &str) -> Result<Rc<Mesh>, MeshError> {
        match self.mesh_map.get(id) {
            Some(m) => Ok(m.clone()),
            _ => Err(MeshError::MeshNotFound(id.to_string()))
        }
    }
    pub fn get_mesh(&self, id: &str) -> Result<&Mesh, MeshError> {
        match self.mesh_map.get(id) {
            Some(m) => Ok(m),
            _ => Err(MeshError::MeshNotFound(id.to_string()))
        }
    }
}

impl Default for MeshManager {
    fn default() -> Self {
        Self {
            mesh_map: BTreeMap::new()
        }
    }
}
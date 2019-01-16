use std::collections::BTreeMap;

pub struct MeshManager {
    mesh_map: BTreeMap<String, Rc<Mesh>>
}

impl MeshManager {
    
}

impl Default for MeshManager {
    fn default() -> Self {
        Self {
            mesh_map: BTreeMap::new()
        }
    }
}
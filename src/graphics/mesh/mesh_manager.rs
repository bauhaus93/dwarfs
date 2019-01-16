use std::collections::BTreeMap;

pub struct MeshManager {
    mesh_map: BTreeMap<String, Rc<Mesh>>
}

impl Default for MeshManager {
    fn default() -> Self {
        Self {
            mesh_map: BTreeMap::new()
        }
    }
}
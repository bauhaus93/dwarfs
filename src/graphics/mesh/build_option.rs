use glm::Vector3;

pub enum BuildOption {
    RemoveIncident,
    RemoveByDirection(Vector3<f32>)
}
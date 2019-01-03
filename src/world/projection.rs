
pub enum Projection {
    Perspective { fov: f32, aspect_ratio: f32, near: f32, far: f32 },
    Orthographic { left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32 }
}

pub fn create_orthographic_projection(size: f32) -> Projection {
    Projection::Orthographic {
        left: -size,
        right: size,
        top: size,
        bottom: -size,
        near: -size,
        far: size
    }
}

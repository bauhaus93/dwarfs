
pub enum Projection {
    Perspective { fov: f32, aspect_ratio: f32, near: f32, far: f32 },
    Orthographic { width: f32, aspect_ratio: f32 }
}

pub fn create_orthographic_projection(width: f32, aspect_ratio: f32) -> Projection {
    Projection::Orthographic {
        width: width,
        aspect_ratio: aspect_ratio,
    }
}


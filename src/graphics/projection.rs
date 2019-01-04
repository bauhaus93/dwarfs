
pub enum Projection {
    Perspective { fov: f32, aspect_ratio: f32, near: f32, far: f32 },
    Orthographic { left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32 }
}

pub fn create_orthographic_projection(width: f32, height: f32) -> Projection {
    Projection::Orthographic {
        left: -width,
        right: width,
        top: width,
        bottom: -width,
        near: -height,
        far: height
    }
}


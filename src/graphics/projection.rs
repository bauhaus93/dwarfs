#[derive(Copy, Clone)]
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

pub fn create_default_perspective() -> Projection {
    Projection::Perspective { fov: 45.0f32.to_radians(), aspect_ratio: 4./3., near: 0.5, far: 500. }
}

pub fn create_default_orthographic() -> Projection {
    Projection::Orthographic { width: 20., aspect_ratio: 4./3. }
}




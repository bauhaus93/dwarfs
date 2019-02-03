use std::ops::Add;
use glm::{ Vector3, Matrix4 };
use glm::ext::{ look_at, perspective };
use num_traits::One;

use graphics::{ Projection, create_direction, create_orthographic_projection, create_orthographic_projection_matrix, projection::{ create_default_orthographic, create_default_perspective } };
use world::{ Model };
use utility::traits::{ Translatable, Rotatable };
use utility::Float;

pub struct Camera {
    model: Model,
    projection: Projection,
    view_matrix: Matrix4<Float>,
    projection_matrix: Matrix4<Float>
}

impl Camera {
     pub fn create_mvp_matrix(&self, model: &Model) -> Matrix4<Float> {
        self.projection_matrix * self.view_matrix * model.get_matrix()
    }

    pub fn zoom(&mut self, factor: Float) {
        match &mut self.projection {
            Projection::Orthographic { width, .. } => { 
                *width = Float::max(Float::min(*width * factor, 1e3), 2e0);
            },
            Projection::Perspective { fov, ..} => {
                *fov = Float::max(Float::min((*fov * factor).to_degrees(), 179.), 1.).to_radians()
            }
        }
        self.update_projection();
    }

    pub fn set_projection(&mut self, new_projection: Projection) {
        self.projection = new_projection;
        self.update_projection();
    }

    pub fn get_projection(&self) -> Projection {
        self.projection
    }

    fn update_view(&mut self) {
        let direction = create_direction(self.model.get_rotation());
        self.view_matrix = look_at(
            self.model.get_translation(),
            self.model.get_translation().add(direction),
            Vector3::<Float>::new(0., 0., 1.));
    }

    fn update_projection(&mut self) {
        self.projection_matrix = match self.projection {
            Projection::Perspective { fov, aspect_ratio, near, far } => {
                trace!("projection update: perspective, fov = {}, aspect ratio = {}, near = {}, far = {}", fov.to_degrees(), aspect_ratio, near, far);
                perspective(fov, aspect_ratio, near, far)
            },
            Projection::Orthographic { width, aspect_ratio } => {
                trace!("projection update: orthographic, width = {}, aspect ratio = {}", width, aspect_ratio);
                create_orthographic_projection_matrix(-width / 2., width / 2., width / 2. / aspect_ratio, -width / 2. / aspect_ratio, -2. * width, 2. * width)
            }
        }
    }

}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            model: Model::default(),
            projection:create_default_orthographic(),
            //projection:  create_default_perspective(),
            view_matrix: Matrix4::<Float>::one(),
            projection_matrix: Matrix4::<Float>::one()
        };
        camera.set_rotation(Vector3::new(45f32.to_radians(), 125f32.to_radians(), 0.));
        camera.update_projection();
        camera
    }
}

impl Translatable for Camera {
    fn set_translation(&mut self, new_translation: Vector3<Float>) {
        self.model.set_translation(new_translation);
        self.update_view();
    }
    fn get_translation(&self) -> Vector3<Float> {
        self.model.get_translation()
    }
}

impl Rotatable for Camera {
    fn set_rotation(&mut self, new_rotation: Vector3<Float>) {
        const MAX_Y: Float = std::f32::consts::PI as Float - 0.01;
        const MIN_Y: Float = 0.01;
        const DOUBLE_PI: Float = 2. * std::f32::consts::PI as Float;
        let mut fixed_rotation = new_rotation;
        if fixed_rotation.x >= DOUBLE_PI {
            fixed_rotation.x -= DOUBLE_PI;
        } else if fixed_rotation.x < 0. {
            fixed_rotation.x += DOUBLE_PI;
        }
        if fixed_rotation.y > MAX_Y {
                fixed_rotation.y = MAX_Y;
        } else if fixed_rotation.y < MIN_Y {
            fixed_rotation.y = MIN_Y;
        }
        self.model.set_rotation(fixed_rotation);
        self.update_view();
    }
    fn get_rotation(&self) -> Vector3<Float> {
        self.model.get_rotation()
    }
}

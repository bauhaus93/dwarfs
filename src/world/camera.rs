use std::ops::Add;
use glm::{ Vector3, Matrix4 };
use glm::ext::{ look_at, perspective };
use gl::types::GLfloat;
use num_traits::One;

use graphics::{ Projection, create_direction, create_orthographic_projection, create_orthographic_projection_matrix };
use world::{ Model };
use utility::traits::{ Translatable, Rotatable };

pub struct Camera {
    model: Model,
    projection: Projection,
    view_matrix: Matrix4<GLfloat>,
    projection_matrix: Matrix4<GLfloat>
}

impl Camera {
     pub fn create_mvp_matrix(&self, model: &Model) -> Matrix4<GLfloat> {
        self.projection_matrix * self.view_matrix * model.get_matrix()
    }

    pub fn zoom(&mut self, factor: f32) {
        match &mut self.projection {
            Projection::Orthographic { width, .. } => { 
                *width = f32::max(f32::min(*width * factor, 1e3), 2e0);
            },
            Projection::Perspective { fov, ..} => {
                *fov = f32::max(f32::min((*fov * factor).to_degrees(), 179f32), 1f32).to_radians()
            }
        }
        self.update_projection();
    }

    pub fn set_projection(&mut self, new_projection: Projection) {
        self.projection = new_projection;
        self.update_projection();
    }

    fn update_view(&mut self) {
        let direction = create_direction(self.model.get_rotation());
        self.view_matrix = look_at(
            self.model.get_translation(),
            self.model.get_translation().add(direction),
            Vector3::<f32>::new(0., 0., 1.));
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
            projection: Projection::Orthographic { width: 20., aspect_ratio: 4./3. },
            //projection:  Projection::Perspective { fov: 75.0f32.to_radians(), aspect_ratio: 4./3., near: 0.5, far: 100. },
            view_matrix: Matrix4::<GLfloat>::one(),
            projection_matrix: Matrix4::<GLfloat>::one()
        };
        camera.mod_translation(Vector3::new(0., 0., 1.));
        camera.set_rotation(Vector3::new(45f32.to_radians(), 125f32.to_radians(), 0.));
        camera.update_projection();
        camera
    }
}

impl Translatable for Camera {
    fn set_translation(&mut self, new_translation: Vector3<f32>) {
        self.model.set_translation(new_translation);
        self.update_view();
    }
    fn get_translation(&self) -> Vector3<f32> {
        self.model.get_translation()
    }
}

impl Rotatable for Camera {
    fn set_rotation(&mut self, new_rotation: Vector3<f32>) {
        const MAX_Y: f32 = std::f32::consts::PI - 0.01;
        const MIN_Y: f32 = 0.01;
        const DOUBLE_PI: f32 = 2. * std::f32::consts::PI;
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
    fn get_rotation(&self) -> Vector3<f32> {
        self.model.get_rotation()
    }
}

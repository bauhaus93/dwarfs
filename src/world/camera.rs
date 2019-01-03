use std::ops::Add;
use glm::{ Vector3, Matrix4 };
use glm::ext::{ look_at, perspective };
use gl::types::GLfloat;
use num_traits::One;

use super::{ Model, Positionable, Rotatable, create_direction };

pub struct Camera {
    model: Model,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    view: Matrix4<GLfloat>,
    projection: Matrix4<GLfloat>
}

impl Camera {
     pub fn create_mvp_matrix(&self, model: &Model) -> Matrix4<GLfloat> {
        self.projection * self.view * model.get_matrix()
    }

    fn update_view(&mut self) {
        let direction = create_direction(self.model.get_rotation());
        self.view = look_at(
            self.model.get_position(),
            self.model.get_position().add(direction),
            Vector3::<f32>::new(0., 0., 1.));
    }

    fn update_projection(&mut self) {
        self.projection = perspective(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far);
        info!("projection update: fov = {}, aspect ratio = {}, near = {}, far = {}", self.fov, self.aspect_ratio, self.near, self.far);
    }

}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            model: Model::default(),
            fov: 75.0,
            aspect_ratio: 4.0 / 3.0,
            near: 0.5,
            far: 100.0,
            view: Matrix4::<GLfloat>::one(),
            projection: Matrix4::<GLfloat>::one()
        };
        camera.mod_position(Vector3::new(0., 0., 5.));
        camera.set_rotation(Vector3::new(0., 180f32.to_radians(), 0.));
        camera.update_projection();
        camera
    }
}

impl Positionable for Camera {
    fn set_position(&mut self, new_position: Vector3<f32>) {
        self.model.set_position(new_position);
        self.update_view();
    }
    fn get_position(&self) -> Vector3<f32> {
        self.model.get_position()
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

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gl;
extern crate glutin;
extern crate glm;
extern crate image;
extern crate num_traits;

pub mod application;
mod graphics;
mod world;
mod utility;

pub use application::Application;

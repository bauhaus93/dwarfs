#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gl;
extern crate glutin;

pub mod application;
mod application_error;
mod graphics;
mod utility;

pub use self::application::Application;

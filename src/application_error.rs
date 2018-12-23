use std::fmt;
use std::error::Error;

use super::graphics;

#[derive(Debug)]
pub enum ApplicationError {
    Graphics(graphics::GraphicsError),
}

impl From<graphics::GraphicsError> for ApplicationError {
    fn from(err: graphics::GraphicsError) -> ApplicationError {
        ApplicationError::Graphics(err)
    }
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Graphics(_) => "graphics",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::Graphics(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}

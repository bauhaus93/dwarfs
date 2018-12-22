use std::fmt;
use std::error::Error;

use super::graphics;

#[derive(Debug)]
pub enum ApplicationError {
    GraphicsError(graphics::GraphicsError),
}

impl From<graphics::GraphicsError> for ApplicationError {
    fn from(err: graphics::GraphicsError) -> ApplicationError {
        ApplicationError::GraphicsError(err)
    }
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::GraphicsError(_) => "graphics error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::GraphicsError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::GraphicsError(ref err) => write!(f, "{}: {}", err.description(), err),
        }
    }
}

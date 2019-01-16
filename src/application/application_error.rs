use std::fmt;
use std::error::Error;
use std::io;

use utility;
use graphics;

#[derive(Debug)]
pub enum ApplicationError {
    Graphics(graphics::GraphicsError),
    File(utility::FileError)
}

impl From<graphics::GraphicsError> for ApplicationError {
    fn from(err: graphics::GraphicsError) -> Self {
        ApplicationError::Graphics(err)
    }
}

impl From<utility::FileError> for ApplicationError {
    fn from(err: utility::FileError) -> Self {
        ApplicationError::File(err)
    }
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Graphics(_) => "graphics",
            ApplicationError::File(_) => "file"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::Graphics(ref err) => Some(err),
            ApplicationError::File(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::File(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}

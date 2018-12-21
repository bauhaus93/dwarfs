use std::fmt;
use std::error::Error;

use glutin;

#[derive(Debug)]
pub enum ApplicationError {
    GlutinCreationError(glutin::CreationError),
    GlutinContextError(glutin::ContextError)
}

impl From<glutin::CreationError> for ApplicationError {
    fn from(err: glutin::CreationError) -> ApplicationError {
        ApplicationError::GlutinCreationError(err)
    }
}

impl From<glutin::ContextError> for ApplicationError {
    fn from(err: glutin::ContextError) -> ApplicationError {
        ApplicationError::GlutinContextError(err)
    }
}



impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::GlutinCreationError(_) => "glutin creation error",
            ApplicationError::GlutinContextError(_) => "glutin context error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::GlutinCreationError(ref err) => Some(err),
            ApplicationError::GlutinContextError(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::GlutinCreationError(ref err) => write!(f, "{}: {}", err.description(), err),
            ApplicationError::GlutinContextError(ref err) => write!(f, "{}: {}", err.description(), err),
        }
    }
}
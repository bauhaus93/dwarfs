use std::fmt;
use std::error::Error;

use glutin;

use super::shader_error::ShaderError;

#[derive(Debug)]
pub enum GraphicsError {
    GlutinCreationError(glutin::CreationError),
    GlutinContextError(glutin::ContextError),
    ShaderError(ShaderError)
}

impl From<glutin::CreationError> for GraphicsError {
    fn from(err: glutin::CreationError) -> GraphicsError {
        GraphicsError::GlutinCreationError(err)
    }
}

impl From<glutin::ContextError> for GraphicsError {
    fn from(err: glutin::ContextError) -> GraphicsError {
        GraphicsError::GlutinContextError(err)
    }
}

impl From<ShaderError> for GraphicsError {
    fn from(err: ShaderError) -> GraphicsError {
        GraphicsError::ShaderError(err)
    }
}



impl Error for GraphicsError {

    fn description(&self) -> &str {
        match *self {
            GraphicsError::GlutinCreationError(_) => "glutin creation error",
            GraphicsError::GlutinContextError(_) => "glutin context error",
            GraphicsError::ShaderError(_) => "shader error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GraphicsError::GlutinCreationError(ref err) => Some(err),
            GraphicsError::GlutinContextError(ref err) => Some(err),
            GraphicsError::ShaderError(ref err) => Some(err)
        }
    }
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GraphicsError::GlutinCreationError(ref err) => write!(f, "{}: {}", err.description(), err),
            GraphicsError::GlutinContextError(ref err) => write!(f, "{}: {}", err.description(), err),
            GraphicsError::ShaderError(ref err) => write!(f, "{}: {}", err.description(), err)
        }
    }
}

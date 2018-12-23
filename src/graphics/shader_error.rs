use std::fmt;
use std::error::Error;
use std::io;

use gl::types::GLuint;

#[derive(Debug)]
pub enum ShaderError {
    IOError(io::Error),
    UnknownShaderType(GLuint)
}

impl From<io::Error> for ShaderError {
    fn from(err: io::Error) -> ShaderError {
        ShaderError::IOError(err)
    }
}

impl Error for ShaderError {

    fn description(&self) -> &str {
        match *self {
            ShaderError::IOError(_) => "io error",
            ShaderError::UnknownShaderType(_) => "unknown shader type"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ShaderError::IOError(ref err) => Some(err),
            ShaderError::UnknownShaderType(_) => None
        }
    }
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderError::IOError(ref err) => write!(f, "{}: {}", self.description(), err),
            ShaderError::UnknownShaderType(type_id) => write!(f, "{}: type id is {}", self.description(), type_id),
        }
    }
}



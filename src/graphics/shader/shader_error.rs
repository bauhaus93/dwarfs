use std::fmt;
use std::error::Error;
use std::io;

use gl::types::{ GLuint, GLint };

use graphics::OpenglError;

#[derive(Debug)]
pub enum ShaderError {
    IO(io::Error),
    UnknownShaderType(GLuint),
    Compilation(String),
    Opengl(OpenglError),
    FunctionFailure(String)
}

impl From<io::Error> for ShaderError {
    fn from(err: io::Error) -> ShaderError {
        ShaderError::IO(err)
    }
}

impl From<OpenglError> for ShaderError {
    fn from(err: OpenglError) -> ShaderError {
        ShaderError::Opengl(err)
    }
}

impl Error for ShaderError {

    fn description(&self) -> &str {
        match *self {
            ShaderError::IO(_) => "io",
            ShaderError::UnknownShaderType(_) => "unknown shader type",
            ShaderError::Compilation(_) => "compilation",
            ShaderError::Opengl(_) => "opengl",
            ShaderError::FunctionFailure(_) => "function failure"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ShaderError::IO(ref err) => Some(err),
            ShaderError::UnknownShaderType(_) => None,
            ShaderError::Compilation(_) => None,
            ShaderError::Opengl(ref err) => Some(err),
            ShaderError::FunctionFailure(_) => None
        }
    }
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderError::IO(ref err) => write!(f, "{}: {}", self.description(), err),
            ShaderError::UnknownShaderType(type_id) => write!(f, "{}: type id is {}", self.description(), type_id),
            ShaderError::Compilation(ref shader_log) => write!(f, "{}: {}", self.description(), shader_log),
            ShaderError::Opengl(ref err) => write!(f, "{}/{}", self.description(), err),
            ShaderError::FunctionFailure(ref func_name) => write!(f, "{} @ {}", self.description(), func_name)
        }
    }
}



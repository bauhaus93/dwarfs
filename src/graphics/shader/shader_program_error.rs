use std::fmt;
use std::error::Error;

use gl;
use gl::types::{ GLuint, GLint };

use graphics::OpenglError;

#[derive(Debug)]
pub enum ShaderProgramError {
    Linkage(String),
    Opengl(OpenglError),
    FunctionFailure(String)
}

impl From<OpenglError> for ShaderProgramError {
    fn from(err: OpenglError) -> ShaderProgramError {
        ShaderProgramError::Opengl(err)
    }
}

impl Error for ShaderProgramError {

    fn description(&self) -> &str {
        match *self {
            ShaderProgramError::Linkage(_) => "linkage",
            ShaderProgramError::Opengl(_) => "opengl",
            ShaderProgramError::FunctionFailure(_) => "function failure"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ShaderProgramError::Linkage(_) => None,
            ShaderProgramError::Opengl(ref err) => Some(err),
            ShaderProgramError::FunctionFailure(_) => None
        }
    }
}

impl fmt::Display for ShaderProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderProgramError::Linkage(ref program_log) => write!(f, "{}: {}", self.description(), program_log),
            ShaderProgramError::Opengl(ref err) => write!(f, "{}/{}", self.description(), err),
            ShaderProgramError::FunctionFailure(ref func_name) => write!(f, "{} @ {}", self.description(), func_name)
        }
    }
}


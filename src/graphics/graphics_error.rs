use std::fmt;
use std::error::Error;

use glutin;

use super::ShaderError;
use super::ShaderProgramError;
use super::OpenglError;

#[derive(Debug)]
pub enum GraphicsError {
    GlutinCreation(glutin::CreationError),
    GlutinContext(glutin::ContextError),
    Shader(ShaderError),
    ShaderProgram(ShaderProgramError),
    Opengl(OpenglError),
    FunctionFailure(String)
}

impl From<glutin::CreationError> for GraphicsError {
    fn from(err: glutin::CreationError) -> GraphicsError {
        GraphicsError::GlutinCreation(err)
    }
}

impl From<glutin::ContextError> for GraphicsError {
    fn from(err: glutin::ContextError) -> GraphicsError {
        GraphicsError::GlutinContext(err)
    }
}

impl From<ShaderError> for GraphicsError {
    fn from(err: ShaderError) -> GraphicsError {
        GraphicsError::Shader(err)
    }
}

impl From<ShaderProgramError> for GraphicsError {
    fn from(err: ShaderProgramError) -> GraphicsError {
        GraphicsError::ShaderProgram(err)
    }
}

impl From<OpenglError> for GraphicsError {
    fn from(err: OpenglError) -> GraphicsError {
        GraphicsError::Opengl(err)
    }
}
impl Error for GraphicsError {

    fn description(&self) -> &str {
        match *self {
            GraphicsError::GlutinCreation(_) => "glutin creation",
            GraphicsError::GlutinContext(_) => "glutin context",
            GraphicsError::Shader(_) => "shader",
            GraphicsError::ShaderProgram(_) => "shader program",
            GraphicsError::Opengl(_) => "opengl",
            GraphicsError::FunctionFailure(_) => "function failure"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GraphicsError::GlutinCreation(ref err) => Some(err),
            GraphicsError::GlutinContext(ref err) => Some(err),
            GraphicsError::Shader(ref err) => Some(err),
            GraphicsError::ShaderProgram(ref err) => Some(err),
            GraphicsError::Opengl(ref err) => Some(err),
            GraphicsError::FunctionFailure(_) => None
        }
    }
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GraphicsError::GlutinCreation(ref err) => write!(f, "{}: {}", self.description(), err),
            GraphicsError::GlutinContext(ref err) => write!(f, "{}: {}", self.description(), err),
            GraphicsError::Shader(ref err) => write!(f, "{}/{}", self.description(), err),
            GraphicsError::ShaderProgram(ref err) => write!(f, "{}/{}", self.description(), err),
            GraphicsError::Opengl(ref err) => write!(f, "{}/{}", self.description(), err),
            GraphicsError::FunctionFailure(ref func_name) => write!(f, "{} @ {}", self.description(), func_name)
        }
    }
}

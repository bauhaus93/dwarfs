use std::fmt;
use std::error::Error;

use gl;
use gl::types::{ GLuint, GLint };

use super::OpenglError;

#[derive(Debug)]
pub enum ShaderProgramError {
    Linkage(GLuint),
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
            ShaderProgramError::Linkage(program_id) => write!(f, "{}: {}", self.description(), get_program_log(program_id)),
            ShaderProgramError::Opengl(ref err) => write!(f, "{}/{}", self.description(), err),
            ShaderProgramError::FunctionFailure(ref func_name) => write!(f, "{} @ {}", self.description(), func_name)
        }
    }
}

fn get_program_log(program_id: GLuint) -> String {
    trace!("getting program log");
    let mut log_len: GLint = 0;
    let mut bytes_written: GLint = 0;
    let mut log_vec: Vec<u8> = Vec::new();
    unsafe {
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_len);
        log_vec.reserve(log_len as usize);
        trace!("allocated log size: {}", log_vec.capacity()); 
        gl::GetProgramInfoLog(program_id, log_vec.capacity() as i32, &mut bytes_written, log_vec.as_mut_ptr() as *mut _);
        log_vec.set_len(bytes_written as usize);
        trace!("log bytes written: {}", bytes_written);
    };
    match String::from_utf8(log_vec) {
        Ok(s) => s,
        Err(_) => "couldn't convert program log".to_string()
    }
}

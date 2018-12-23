use std::fmt;
use std::error::Error;
use std::io;

use gl::types::{ GLuint, GLint };

#[derive(Debug)]
pub enum ShaderError {
    IO(io::Error),
    UnknownShaderType(GLuint),
    Compilation(GLuint),
}

impl From<io::Error> for ShaderError {
    fn from(err: io::Error) -> ShaderError {
        ShaderError::IO(err)
    }
}

impl Error for ShaderError {

    fn description(&self) -> &str {
        match *self {
            ShaderError::IO(_) => "io",
            ShaderError::UnknownShaderType(_) => "unknown shader type",
            ShaderError::Compilation(_) => "compilation",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ShaderError::IO(ref err) => Some(err),
            ShaderError::UnknownShaderType(_) => None,
            ShaderError::Compilation(_) => None,
        }
    }
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderError::IO(ref err) => write!(f, "{}: {}", self.description(), err),
            ShaderError::UnknownShaderType(type_id) => write!(f, "{}: type id is {}", self.description(), type_id),
            ShaderError::Compilation(shader_id) => write!(f, "{}: {}", self.description(), get_shader_log(shader_id)),
        }
    }
}

fn get_shader_log(shader_id: GLuint) -> String {
    trace!("getting shader log");
    let mut log_len: GLint = 0;
    let mut bytes_written: GLint = 0;
    let mut log_vec: Vec<u8> = Vec::new();
    unsafe {
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_len);
        log_vec.reserve(log_len as usize);
        trace!("allocated log size: {}", log_vec.capacity()); 
        gl::GetShaderInfoLog(shader_id, log_vec.capacity() as i32, &mut bytes_written, log_vec.as_mut_ptr() as *mut _);
        log_vec.set_len(bytes_written as usize);
        trace!("log bytes written: {}", bytes_written);
    };
    match String::from_utf8(log_vec) {
        Ok(s) => s,
        Err(_) => "couldn't convert shader log".to_string()
    }
}


use std::ffi::CStr;
use std::string::FromUtf8Error;

use gl;
use gl::types::GLenum;

pub fn get_opengl_version() -> Result<String, FromUtf8Error> {
    opengl_get_string(gl::VERSION)
}

fn opengl_get_string(name: GLenum) -> Result<String, FromUtf8Error> {
    let data_vec = unsafe {
        CStr::from_ptr(gl::GetString(name) as *const _).to_bytes().to_vec()
    };
    Ok(String::from_utf8(data_vec)?)
}



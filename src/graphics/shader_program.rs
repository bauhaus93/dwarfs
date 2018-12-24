use gl;
use gl::types::{ GLint, GLuint };

use super::ShaderProgramError;
use super::OpenglError;

pub struct ShaderProgram {
    id: GLuint, 
    mvp_handle: GLint
}

impl ShaderProgram {
    
    pub fn new(program_id: GLuint) -> Result<ShaderProgram, ShaderProgramError> {
        debug_assert!(program_id != 0);
        let mvp_handle = match get_resource_handle(program_id, "MVP") {
            Ok(h) => h,
            Err(e) => {
                unsafe {
                    gl::DeleteProgram(program_id);
                }
                return Err(e);
            }
        };
        Ok(ShaderProgram {
            id: program_id,
            mvp_handle: mvp_handle
        })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        if self.id != 0 {
            debug!("Deleting shader program");
            unsafe {
                gl::DeleteProgram(self.id);
            }
            self.id = 0;
        }
    }
}

fn get_resource_handle(program_id: GLuint, resource_name: &str) -> Result<GLint, ShaderProgramError> {
    let res_name_zero_term = resource_name.to_string() + "\0";
    let handle: GLint = unsafe {
        gl::GetUniformLocation(program_id, res_name_zero_term.as_ptr() as *const _)
    };
    if handle == -1 {
        OpenglError::check_error("gl::GetUniformLocation")?;
        return Err(ShaderProgramError::FunctionFailure("gl::GetUniformLocation".to_string()));
    }
    Ok(handle)
}

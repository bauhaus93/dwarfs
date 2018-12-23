use gl;
use gl::types::GLuint;

pub struct ShaderProgram {
    id: GLuint 

}

impl ShaderProgram {

    pub fn new(program_id: GLuint) -> ShaderProgram {
        debug_assert!(program_id != 0);
        ShaderProgram {
            id: program_id
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}



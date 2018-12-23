use std::ptr;
use gl;
use gl::types::{ GLuint, GLenum };

use utility::read_file;
use super::ShaderProgram;
use super::ShaderError;

pub struct ShaderProgramBuilder {
    shader_list: Vec<Shader>
}

struct Shader {
    shader_type: GLenum,
    shader_file_path: String
}


impl ShaderProgramBuilder {


    pub fn new() -> ShaderProgramBuilder {
        ShaderProgramBuilder {
            shader_list: Vec::new()
        }
    }

    pub fn add_shader(mut self, shader_type: GLenum, shader_file_path: &str) -> Self {
        let shader = Shader {
            shader_type: shader_type,
            shader_file_path: shader_file_path.to_string()
        };
        self.shader_list.push(shader);
        self
    }

    pub fn finish(mut self) -> Result<ShaderProgram, ShaderError> {
        let shader_ids = compile_shaders(self.shader_list)?;
    
        debug!("Creating shader program");
        let program_id = unsafe {
            let program_id = gl::CreateProgram();
            for shader_id in shader_ids {
                gl::AttachShader(program_id, shader_id);
            }
            gl::LinkProgram(program_id);
            program_id
        };
        let program = ShaderProgram::new(program_id);
        Ok(program)
    }

}

fn compile_shaders(shader_list: Vec<Shader>) -> Result<Vec<GLuint>, ShaderError> {
    let mut shader_ids: Vec<GLuint> = Vec::new();
    for shader in shader_list {
        let shader_id = compile_shader(shader)?;
        shader_ids.push(shader_id);
    
    }
    Ok(shader_ids)
}

fn compile_shader(shader: Shader) -> Result<GLuint, ShaderError> {
    let shader_name = match shader.shader_type {
        gl::FRAGMENT_SHADER => "fragment shader",
        gl::VERTEX_SHADER => "vertex shader",
        unknown_type => { return Err(ShaderError::UnknownShaderType(unknown_type)); }
    };
    debug!("Compiling {}", shader_name);
    trace!("Reading shader file '{}'", shader.shader_file_path);
    let source = read_file(&shader.shader_file_path)?;
    let shader_id = unsafe {
        let id = gl::CreateShader(shader.shader_type);
        gl::ShaderSource(id, 1, [source.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(id);
        id
    };
    debug!("Compiled shader");
    Ok(shader_id)
}




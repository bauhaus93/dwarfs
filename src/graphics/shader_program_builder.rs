use std::ptr;
use gl;
use gl::types::{ GLuint, GLint, GLenum };

use utility::read_file;
use super::ShaderProgram;
use super::ShaderError;
use super::ShaderProgramError;
use super::GraphicsError;
use super::OpenglError;

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

    pub fn add_vertex_shader(self, shader_file_path: &str) -> Self {
        self.add_shader(gl::VERTEX_SHADER, shader_file_path)
    }

    pub fn add_fragment_shader(self, shader_file_path: &str) -> Self {
        self.add_shader(gl::FRAGMENT_SHADER, shader_file_path)
    }

    fn add_shader(mut self, shader_type: GLenum, shader_file_path: &str) -> Self {
        let shader = Shader {
            shader_type: shader_type,
            shader_file_path: shader_file_path.to_string()
        };
        self.shader_list.push(shader);
        self
    }
    pub fn finish(self) -> Result<ShaderProgram, GraphicsError> {
        info!("Creating shader program, using {} shader/s", self.shader_list.len());
        let shader_ids = compile_shaders(self.shader_list)?; 
        let program_id = unsafe {
            gl::CreateProgram()
        };
        if program_id == 0 {
            delete_shaders(shader_ids);
            OpenglError::check_error("gl::CreateProgram")?;
            return Err(GraphicsError::FunctionFailure("gl::CreateProgram".to_string()));
        }

        for shader_id in &shader_ids {
            unsafe {
               gl::AttachShader(program_id, *shader_id);
            }
        }
        match OpenglError::check_error("gl::AttachShader") {
            Ok(_) => { },
            Err(e) => {
                cleanup_shader_program(program_id, shader_ids);
                return Err(GraphicsError::from(e));
            }
        }

        debug!("Linking shader program");
        unsafe {
            gl::LinkProgram(program_id);
        }
        match OpenglError::check_error("gl::LinkProgram") {
            Ok(_) => { },
            Err(e) => {
                cleanup_shader_program(program_id, shader_ids);
                return Err(GraphicsError::from(e));
            }
        }

        for shader_id in &shader_ids {
            unsafe {
                gl::DetachShader(program_id, *shader_id);
            }
        }
        match OpenglError::check_error("gl::DetachShader") {
            Ok(_) => { },
            Err(e) => {
                cleanup_shader_program(program_id, shader_ids);
                return Err(GraphicsError::from(e));
            }
        }
            
        delete_shaders(shader_ids);

        let mut success: GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }
        match OpenglError::check_error("gl::GetProgramiv") {
            Ok(_) => { },
            Err(e) => {
                delete_program(program_id);
                return Err(GraphicsError::from(e));
            }
        }
        if success == 0 {
            let err = Err(GraphicsError::from(ShaderProgramError::Linkage(program_id)));
            delete_program(program_id);
            return err;
        }
        let program = ShaderProgram::new(program_id)?;
        Ok(program)
    }
}

fn compile_shaders(shader_list: Vec<Shader>) -> Result<Vec<GLuint>, ShaderError> {
    let mut shader_ids: Vec<GLuint> = Vec::new();
    for shader in shader_list {
        let shader_id = match compile_shader(shader) {
            Ok(s) => s,
            Err(e) => {
                    delete_shaders(shader_ids);
                    return Err(e);
                }
        };
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
    let source = read_file(&shader.shader_file_path)? + "\0";
    let shader_id = unsafe {
        let id = gl::CreateShader(shader.shader_type);
        gl::ShaderSource(id, 1, [source.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(id);
        let mut success: GLint = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let err = Err(ShaderError::Compilation(id));
            gl::DeleteShader(id);
            return err;
        }        
        id
    };
    Ok(shader_id)
}

fn cleanup_shader_program(program_id: GLuint, shader_ids: Vec<GLuint>) {
    detach_attached_shaders(program_id);
    delete_shaders(shader_ids);
    delete_program(program_id);
}

fn delete_program(program_id: GLuint) {
    debug_assert!(program_id != 0);
    unsafe {
        gl::DeleteProgram(program_id);
    }
}

fn delete_shaders(shader_ids: Vec<GLuint>) {
    unsafe {
        for id in shader_ids {
            gl::DeleteShader(id);
        }
    }
}

fn detach_attached_shaders(program_id: GLuint) {
    let attach_count = unsafe {
        let mut count: GLint = 0;
        gl::GetProgramiv(program_id, gl::ATTACHED_SHADERS, &mut count);
        count
    };
    let shader_ids = unsafe {
        let mut ids: Vec<GLuint> = Vec::with_capacity(attach_count as usize);
        ids.set_len(attach_count as usize);
        gl::GetAttachedShaders(program_id, attach_count, ptr::null_mut(), ids.as_ptr() as * mut _);
        ids 
    };
    unsafe {
        for id in shader_ids {
            gl::DetachShader(program_id, id);
        }
    }
}

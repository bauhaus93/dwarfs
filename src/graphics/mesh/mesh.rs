use gl; 
use gl::types::{ GLuint };

use graphics::{ check_opengl_error };
use super::{ Triangle, Vertex };

pub struct Mesh {
    vao: GLuint,
    vbos: [GLuint; 4],
    index_count: GLuint,
    triangles: Vec<Triangle> 
}

impl Mesh {
    pub fn new(vao: GLuint, vbos: [GLuint; 4], index_count: GLuint, triangles: Vec<Triangle>) -> Self {
        debug_assert!(vao != 0);
        debug_assert!(vbos.iter().all(|v| *v != 0));
        Self {
            vao: vao,
            vbos: vbos,
            index_count: index_count,
            triangles: triangles
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(4, &self.vbos[0] as * const GLuint); }
        match check_opengl_error("gl::DeleteBuffers") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
        unsafe { gl::DeleteVertexArrays(1, &self.vao); }
        match check_opengl_error("gl::DeleteVertexArrays") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
    }
}


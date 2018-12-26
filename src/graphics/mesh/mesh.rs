use gl; 
use gl::types::{ GLuint };

use super::{ Triangle, Vertex };

pub struct Mesh {
    vao: GLuint,
    vertex_buffer: GLuint,
    uv_buffer: GLuint,
    normal_buffer: GLuint,
    index_buffer: GLuint,
    index_count: u32 
}


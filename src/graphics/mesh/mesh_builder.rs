use std::mem::size_of;
use std::ffi::c_void;
use gl;
use gl::types::{ GLfloat, GLuint, GLenum, GLsizeiptr };

use graphics::{ OpenglError, check_opengl_error };
use super::{ Vertex, Triangle, Quad, Mesh };

pub struct MeshBuilder { 
    triangles: Vec<Triangle>,
    indexed_vertices: Vec<(Vertex, GLuint)>,
    position_buffer: Vec<GLfloat>,
    uv_buffer: Vec<GLfloat>,
    normal_buffer: Vec<GLfloat>,
    index_buffer: Vec<GLuint>,
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            triangles: Vec::new(),
            indexed_vertices: Vec::new(),
            position_buffer: Vec::new(),
            uv_buffer: Vec::new(),
            normal_buffer: Vec::new(),
            index_buffer: Vec::new()
        }
    }

    pub fn add_quad(self, quad: Quad) -> Self {
        quad.to_triangles().into_iter().fold(self, |s, t| s.add_triangle(t))
    }

    pub fn add_triangle(mut self, triangle: &Triangle) -> Self {
        for vert in triangle.get_vertices().iter() {
            let new_vert_index = match self.indexed_vertices.iter().find(|(v, _i)| v == vert) {
                Some((_, i)) => {
                    self.index_buffer.push(*i);
                    None
                },
                None => {
                    debug_assert!(self.position_buffer.len() % 3 == 0);
                    debug_assert!(self.uv_buffer.len() % 3 == 0);
                    debug_assert!(self.normal_buffer.len() % 3 == 0);
                    let new_index = (self.position_buffer.len() / 3) as GLuint;
                    self.position_buffer.extend(vert.get_pos());
                    self.uv_buffer.extend(vert.get_uv());
                    self.normal_buffer.extend(vert.get_normal());
                    self.index_buffer.push(new_index);
                    Some((vert.clone(), new_index))
                }
            };
            match new_vert_index {
                Some(vi) => self.indexed_vertices.push(vi),
                None => {}
            }
        }
        self 
    }

    fn load_vbos(&self) -> Result<[GLuint; 4], OpenglError> {
        let mut vbos: [GLuint; 4] = [0; 4];
        
        unsafe {
            gl::GenBuffers(4, &mut vbos[0] as * mut GLuint);
            check_opengl_error("gl::GenBuffers")?;

            match fill_buffer(vbos[0], gl::ARRAY_BUFFER, (self.position_buffer.len() * size_of::<GLfloat>()) as GLsizeiptr, self.position_buffer.as_ptr() as * const _) {
                Ok(_) => {},
                Err(e) => {
                    delete_buffers(vbos);
                    return Err(e);
                }
            }

            match fill_buffer(vbos[1], gl::ARRAY_BUFFER, (self.uv_buffer.len() * size_of::<GLfloat>()) as GLsizeiptr, self.uv_buffer.as_ptr() as * const _) {
                Ok(_) => {},
                Err(e) => {
                    delete_buffers(vbos);
                    return Err(e);
                }
            }

            match fill_buffer(vbos[2], gl::ARRAY_BUFFER, (self.normal_buffer.len() * size_of::<GLfloat>()) as GLsizeiptr, self.normal_buffer.as_ptr() as * const _) {
                Ok(_) => {},
                Err(e) => {
                    delete_buffers(vbos);
                    return Err(e);
                }
            }

            match fill_buffer(vbos[3], gl::ELEMENT_ARRAY_BUFFER, (self.index_buffer.len() * size_of::<GLuint>()) as GLsizeiptr, self.index_buffer.as_ptr() as * const _) {
                Ok(_) => {},
                Err(e) => {
                    delete_buffers(vbos);
                    return Err(e);
                }
            }
        }
    Ok(vbos)
    }

    fn load_vao(&self, vbos: &[GLuint; 4]) -> Result<GLuint, OpenglError> {
        unsafe {
            let mut vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);


            Ok(vao)
        }
    }

}

fn fill_buffer(buffer_id: GLuint, buffer_type: GLenum, buffer_size: GLsizeiptr, buffer_data: * const c_void) -> Result<(), OpenglError> {
    unsafe {
        gl::BindBuffer(buffer_type, buffer_id);
        check_opengl_error("gl::BindBuffer")?;
        gl::BufferData(buffer_type, buffer_size, buffer_data, gl::STATIC_DRAW);
        check_opengl_error("gl::BufferData")?;
    }
    Ok(()) 
}

fn delete_buffers(buffers: [GLuint; 4]) {
    unsafe {
        gl::DeleteBuffers(4, &buffers[0] as * const GLuint);
    }
}


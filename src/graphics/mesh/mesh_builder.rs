use std::mem::size_of;
use std::ffi::c_void;
use std::ptr;
use gl;
use gl::types::{ GLfloat, GLint, GLuint, GLenum, GLsizeiptr };

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
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            indexed_vertices: Vec::new(),
            position_buffer: Vec::new(),
            uv_buffer: Vec::new(),
            normal_buffer: Vec::new(),
            index_buffer: Vec::new()
        }
    }

    pub fn add_quad(mut self, quad: Quad) -> Self {
        for t in &quad.to_triangles() {
            self.add_triangle(t);
        }
        self
    }

    fn add_triangle(&mut self, triangle: &Triangle) {
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
    }

    pub fn finish(self) -> Result<Mesh, OpenglError> {
        let vbos = self.load_vbos()?;
        let vao = match self.load_vao(&vbos) {
            Ok(vao) => vao,
            Err(e) => {
                delete_buffers(vbos);
                return Err(e);
            }
        };
        let mesh = Mesh::new(vao, vbos, self.index_buffer.len() as GLuint, self.triangles);
        Ok(mesh)
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
        let mut vao: GLuint = 0;

        unsafe { gl::GenVertexArrays(1, &mut vao); }
        check_opengl_error("gl::GenVertexArrays")?;

        unsafe { gl::BindVertexArray(vao); }
        match check_opengl_error("gl::BindVertexArray") {
            Ok(_) => {},
            Err(e) => {
                delete_vertex_array(vao);
                return Err(e);
            }
        }

        for (index, vbo) in vbos[..3].iter().enumerate() {
            match assign_buffer_to_vao(*vbo, index as GLuint, 3, gl::FLOAT) {
                Ok(_) => {},
                Err(e) => {
                    delete_vertex_array(vao);
                    return Err(e);
                }
            }
        }

        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbos[3]); }
        match check_opengl_error("gl::BindBuffer") {
            Ok(_) => {},
            Err(e) => {
                delete_vertex_array(vao);
                return Err(e);
            }
        }

        unsafe { gl::BindVertexArray(0); }
        match check_opengl_error("gl::BindVertexArray") {
            Ok(_) => {},
            Err(e) => {
                delete_vertex_array(vao);
                return Err(e);
            }
        }

        for i in 0..3 {
            unsafe { gl::DisableVertexAttribArray(i) }
            match check_opengl_error("gl::DisableVertexAttribArray") {
                Ok(_) => {},
                Err(e) => {
                    delete_vertex_array(vao);
                    return Err(e);
                }
            }
        }
        Ok(vao)
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

fn assign_buffer_to_vao(vbo: GLuint, index: GLuint, size: GLint, data_type: GLenum) -> Result<(), OpenglError> {
    unsafe {
        gl::EnableVertexAttribArray(index);
        check_opengl_error("gl::EnableVertexAttribArray")?;
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        check_opengl_error("gl::BindBuffer")?;
        gl::VertexAttribPointer(index, size, data_type, gl::FALSE, 0, ptr::null());
        check_opengl_error("gl::VertexAttribPointer")?;    
    }
    Ok(())
}

fn delete_vertex_array(vao: GLuint) {
    unsafe {
        gl::DeleteVertexArrays(1, &vao as * const GLuint);
    }
}

fn delete_buffers(buffers: [GLuint; 4]) {
    unsafe {
        gl::DeleteBuffers(4, &buffers[0] as * const GLuint);
    }
}


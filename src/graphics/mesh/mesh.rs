use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::{ ptr, io, ffi::c_void, mem::size_of };
use gl;
use gl::types::{ GLfloat, GLint, GLuint, GLenum, GLsizeiptr };

use utility::read_obj;
use graphics::{ check_opengl_error, OpenglError, mesh::{ Vertex, Triangle } };
use super::MeshError;

pub struct Mesh {
    vao: GLuint,
    vbos: [GLuint; 4],
    index_count: GLuint,
}

impl Mesh {
    pub fn from_obj(obj_path: &str) -> Result<Mesh, MeshError> {
        Self::from_triangles(read_obj(obj_path)?)
    }

    pub fn from_triangles(triangles: Vec<Triangle>) -> Result<Mesh, MeshError> {
        let mut indexed_vertices: BTreeMap<Vertex, GLuint> = BTreeMap::new();
        let mut position_buffer: Vec<GLfloat> = Vec::new();
        let mut uv_buffer: Vec<GLfloat> = Vec::new();
        let mut normal_buffer: Vec<GLfloat> = Vec::new();
        let mut index_buffer: Vec<GLuint> = Vec::new();
        for triangle in triangles.into_iter() {
            for vertex in &triangle.into_vertices() {
                match indexed_vertices.entry(*vertex) {
                    Entry::Occupied(o) => {
                        index_buffer.push(*o.get());
                    },
                    Entry::Vacant(v) => {
                        debug_assert!(position_buffer.len() % 3 == 0);
                        debug_assert!(uv_buffer.len() % 3 == 0);
                        debug_assert!(normal_buffer.len() % 3 == 0);
                        let new_index = (position_buffer.len() / 3) as GLuint;
                        position_buffer.extend(vertex.get_pos().as_array());
                        uv_buffer.extend(vertex.get_uv().as_array());
                        normal_buffer.extend(vertex.get_normal().as_array());
                        index_buffer.push(new_index);
                        v.insert(new_index);
                    }
                }
            }
        }
        let indices = index_buffer.len() as GLuint;
        let vbos = load_vbos(position_buffer, uv_buffer, normal_buffer, index_buffer)?;
        let vao = match load_vao(&vbos) {
            Ok(vao) => vao,
            Err(e) => {
                delete_buffers(vbos);
                return Err(MeshError::from(e));
            }
        };
        Ok(Self {
            vao: vao,
            vbos: vbos,
            index_count: indices
        })
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.index_count as u32
    }

    pub fn render(&self) -> Result<(), MeshError> {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                ptr::null()
            );
        }
        check_opengl_error("Mesh::render")?;
        Ok(())
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

fn load_vbos(position_buffer: Vec<GLfloat>,
             uv_buffer: Vec<GLfloat>,
             normal_buffer: Vec<GLfloat>,
             index_buffer: Vec<GLuint>) -> Result<[GLuint; 4], OpenglError> {
    let mut vbos: [GLuint; 4] = [0; 4];
    
    unsafe { gl::GenBuffers(4, &mut vbos[0] as * mut GLuint) };
    check_opengl_error("gl::GenBuffers")?;

    match fill_buffer(vbos[0], gl::ARRAY_BUFFER, (position_buffer.len() * size_of::<GLfloat>()) as GLsizeiptr, position_buffer.as_ptr() as * const _) {
        Ok(_) => {},
        Err(e) => {
            delete_buffers(vbos);
            return Err(e);
        }
    }

    match fill_buffer(vbos[1], gl::ARRAY_BUFFER, (uv_buffer.len() * size_of::<GLfloat>()) as GLsizeiptr, uv_buffer.as_ptr() as * const _) {
        Ok(_) => {},
        Err(e) => {
            delete_buffers(vbos);
            return Err(e);
        }
    }

    match fill_buffer(vbos[2], gl::ARRAY_BUFFER, (normal_buffer.len() * size_of::<GLfloat>()) as GLsizeiptr, normal_buffer.as_ptr() as * const _) {
        Ok(_) => {},
        Err(e) => {
            delete_buffers(vbos);
            return Err(e);
        }
    }

    match fill_buffer(vbos[3], gl::ELEMENT_ARRAY_BUFFER, (index_buffer.len() * size_of::<GLuint>()) as GLsizeiptr, index_buffer.as_ptr() as * const _) {
        Ok(_) => {},
        Err(e) => {
            delete_buffers(vbos);
            return Err(e);
        }
    }
    Ok(vbos)
}

fn load_vao(vbos: &[GLuint; 4]) -> Result<GLuint, OpenglError> {
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
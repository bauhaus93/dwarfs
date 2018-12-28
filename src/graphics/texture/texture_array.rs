use gl;
use gl::types::GLuint;

use graphics::check_opengl_error;

pub struct TextureArray {
    texture_id: GLuint,
    size: (u32, u32, u32)
}

impl TextureArray {
    pub fn new(texture_id: GLuint, size: (u32, u32, u32)) -> TextureArray {
        debug_assert!(texture_id != 0);
        TextureArray {
            texture_id: texture_id,
            size: size
        }
    }
}

impl Drop for TextureArray {
    fn drop(&mut self) {
        debug!("Deleting texture id = {}", self.texture_id);
        unsafe { gl::DeleteTextures(1, &self.texture_id); }
        match check_opengl_error("gl::DeleteTextures") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
    }
}

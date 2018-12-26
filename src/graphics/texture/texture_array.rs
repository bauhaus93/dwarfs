use gl;
use gl::types::GLuint;

use graphics::check_opengl_error;

pub struct TextureArray {
    texture_id: GLuint,
    size: (u32, u32, u32)
}

impl TextureArray {
    pub fn new(texture_id: GLuint, size: (u32, u32, u32)) -> TextureArray {
        TextureArray {
            texture_id: texture_id,
            size: size
        }
    }
}

impl Drop for TextureArray {
    fn drop(&mut self) {
        if self.texture_id != 0 {
            debug!("Deleting texture id = {}", self.texture_id);
            unsafe {
                gl::DeleteTextures(1, &self.texture_id);
            }
            match check_opengl_error("gl::DeleteTextures") {
                 Ok(_) => {},
                Err(e) => {
                    warn!("{}", e);
                }
            }
        }
    }
}

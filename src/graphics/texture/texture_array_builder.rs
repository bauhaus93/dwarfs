use std::cmp::min;
use gl;
use image;
use image::GenericImageView;

use gl::types::{ GLint, GLuint, GLsizei };

use graphics::{ GraphicsError, OpenglError, check_opengl_error };
use super::TextureArray;

pub struct TextureArrayBuilder {
    atlas_path: String,
    texture_size: (u32, u32),
    texture_origins: Vec<(u32, u32)>
}

impl TextureArrayBuilder {

    pub fn new(atlas_path: &str, texture_size: (u32, u32)) -> TextureArrayBuilder {
        TextureArrayBuilder {
            atlas_path:  atlas_path.to_string(),
            texture_size: texture_size,
            texture_origins: Vec::new()
        }
    }

    pub fn add_texture(mut self, origin: (u32, u32)) -> Self {
        self.texture_origins.push(origin);
        self
    }

    pub fn finish(self) -> Result<TextureArray, GraphicsError> {
        info!("Creating texture array");
        let mipmaps = {
            let dim = min(self.texture_size.0, self.texture_size.1) as f32;
            dim.log(2.0) as u32
        };
        let layer_count: u32 = self.texture_origins.len() as u32;
        debug!("Size = {}x{}x{}, mipmaps = {}", self.texture_size.0, self.texture_size.1, layer_count, mipmaps);
        let texture_id = create_texture(
            (self.texture_size.0 as GLsizei, self.texture_size.1 as GLsizei),
            layer_count as GLsizei,
            mipmaps as GLsizei
        )?;

        debug!("Opening atlas image '{}'", self.atlas_path);
        let img = match image::open(self.atlas_path.clone())? {
            image::DynamicImage::ImageRgba8(img) => img,
            _ => { 
                delete_texture(texture_id);
                return Err(GraphicsError::InvalidImageFormat(self.atlas_path.to_string()));
            } 
        };
        debug!("Adding images to texture");
        match add_subimages(texture_id, img, self.texture_size, &self.texture_origins) {
            Ok(_) => {},
            Err(e) => {
                delete_texture(texture_id);
                return Err(e);
            }
        } 
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0);
            match check_opengl_error("gl::BindTexture") {
                Ok(_) => {},
                Err(e) => {
                    delete_texture(texture_id);
                    return Err(GraphicsError::Opengl(e));
                }
            }
        }
        let texture_array = TextureArray::new(
            texture_id,
            (self.texture_size.0, self.texture_size.1, layer_count) 
        );
        Ok(texture_array)
    }
}

fn create_texture(size: (GLsizei, GLsizei), layers: GLsizei, mipmaps: GLsizei) -> Result<GLuint, OpenglError> {
    debug_assert!(size.0 > 0);
    debug_assert!(size.1 > 0);
    debug_assert!(layers > 0);
    let mut id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
    }
    check_opengl_error("gl::GenTextures")?;
    debug_assert!(id != 0);
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D_ARRAY, id);
    }
    match check_opengl_error("gl::BindTexture") {
        Ok(_) => {},
        Err(e) => {
            delete_texture(id);
            return Err(e);
        }
    }
    unsafe {
        gl::TexStorage3D(
            gl::TEXTURE_2D_ARRAY,
            mipmaps as GLsizei,
            gl::RGBA8,
            size.0,
            size.1,
            layers
        );
    }
    Ok(id)
}

fn add_subimages(texture_id: GLuint, img: image::RgbaImage, sub_size: (u32, u32), sub_origins: &[(u32, u32)])  -> Result<(), GraphicsError>{ 
    for (layer, origin) in sub_origins.iter().enumerate() {
        trace!("Adding subimage, origin = {}/{}", origin.0, origin.1);
        let sub_img = img.view(origin.0, origin.1, sub_size.0, sub_size.1).to_image();
        let pixels: Vec<u8> = sub_img.into_raw();
        add_subimage(
            texture_id,
            (sub_size.0 as GLsizei, sub_size.1 as GLsizei),
            layer as GLsizei,
            &pixels
        )?;
    }
    Ok(())
}

fn add_subimage(texture_id: GLuint, size: (GLsizei, GLsizei), layer: GLsizei, sub_image: &[u8]) -> Result<(), OpenglError> {
    debug_assert!(texture_id != 0);
    trace!("Adding subimage, texture id = {}, size = {}x{}, layer = {}",
        texture_id, size.0, size.1, layer);
    unsafe {
        gl::TexSubImage3D(
            gl::TEXTURE_2D_ARRAY,
            0,
            0, 0, layer,
            size.0, size.1, 1,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            sub_image.as_ptr() as * const _
        );
        check_opengl_error("gl::TexSubImage3D")?;
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        check_opengl_error("gl::TexParameteri")?;
        gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        check_opengl_error("gl::GenerateMipmap")?;
    }
    Ok(())
}
 
fn delete_texture(texture_id: GLuint) {
    debug_assert!(texture_id != 0);
    unsafe {
        gl::DeleteTextures(1, &texture_id);
    }
}

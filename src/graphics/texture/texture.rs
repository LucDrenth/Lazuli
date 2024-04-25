use gl::types::{GLuint, GLenum};
use glam::Vec2;

use crate::{error::opengl, log};

use super::{texture_image::GlTextureImage, TextureImage};

pub trait Texture {
    fn activate(&self, unit: usize);
    fn bind(&self);
    fn size(&self) -> Vec2;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
}

pub struct GlTexture {
    pub id: GLuint,
    original_size: Vec2,
}

impl GlTexture {
    fn create() -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
        }

        opengl::gl_check_errors();
        Self { id, original_size: Vec2::ZERO }
    }

    pub fn new_from_path(path: impl Into<String>) -> Result<Self, String> {
        let mut texture = Self::create();

        let path_string = path.into();

        match image::open(path_string.clone()) {
            Ok(img) => {
                texture.bind();

                let image_size = Self::upload(&img.into_rgba8());
                texture.original_size = image_size;

                Ok(texture)
            }
            Err(err) => {
                Err(format!("Failed to load texture image from path {:?}: {}", path_string, err))
            }
        }
    }

    // Currently always returns Ok, but returns a Result to keep consistent with new_from_path
    pub fn new_from_image<T: Into<GlTextureImage>>(img: T) -> Result<Self, String> {
        let mut texture = Self::create();
        texture.bind();

        let image_size = Self::upload(img);
        texture.original_size = image_size;

        Ok(texture)
    }

    fn upload<T: Into<GlTextureImage>>(texture_image: T) -> Vec2 {
        let img = texture_image.into();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                img.format() as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                img.format(),
                gl::UNSIGNED_BYTE,
                img.bytes() as *const _,
            );
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // Empty space around when texture coordinates are not fully from 0.0 to 1.0
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
            
            // Stretch when texture coordinates are not fully from 0.0 to 1.0
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            // Repeat when texture coordinates are not fully from 0.0 to 1.0
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
            
        opengl::gl_check_errors();

        Vec2 { x: img.width() as f32, y: img.height() as f32 }
    }
}

impl Texture for GlTexture {
    fn activate(&self, unit: usize) {
        unsafe {
            gl::ActiveTexture(to_gl_texture_unit(unit as u32));
        }

        self.bind();
    }

    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }

        opengl::gl_check_errors();
    }

    fn size(&self) -> Vec2 { self.original_size }
    fn width(&self) -> f32 { self.size().x }
    fn height(&self) -> f32 { self.size().y }
}

fn to_gl_texture_unit(unit: u32) -> GLenum {
    // TODO some systems have lower amount of available texture units. Get this dynamically
    let lowest = gl::TEXTURE0;
    let highest = gl::TEXTURE31;

    if lowest + unit > highest {
        log::engine_warn(format!("Texture unit {} is higher than limit {}. Using {} instead.", unit, highest - lowest, highest - lowest));
        return gl::TEXTURE31;
    }

    return lowest + unit;
}

impl Drop for GlTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr())
        }
    }
}

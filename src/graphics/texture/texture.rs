use gl::types::{GLuint, GLenum};
use glam::Vec2;

use crate::{error::opengl, log};

use super::TextureImage;

pub struct Texture {
    pub id: GLuint,
    original_size: Vec2,
}

impl Texture {
    pub fn new() -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
        }

        opengl::gl_check_errors();
        Self { id, original_size: Vec2::ZERO }
    }

    pub fn activate(&self, unit: usize) {
        unsafe {
            gl::ActiveTexture(to_gl_texture_unit(unit as u32));
        }

        self.bind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }

        opengl::gl_check_errors();
    }

    pub fn load_from_path(&mut self, path: impl Into<String>) -> Result<(), String> {
        let path_string = path.into();

        match image::open(path_string.clone()) {
            Ok(img) => {
                let width = img.width() as f32;
                let height = img.height() as f32;

                self.bind();
                Self::upload(&img.into_rgba8());

                self.original_size = Vec2::new(width, height);

                Ok(())
            }
            Err(err) => {
                Err(format!("Failed to load texture image from path {:?}: {}", path_string, err))
            }
        }
    }

    pub fn load_from_image<T: Into<TextureImage>>(&self, img: T) {
        self.bind();
        Self::upload(img);
    }

    fn upload<T: Into<TextureImage>>(texture_image: T) {
        let img: TextureImage = texture_image.into();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                img.format as i32,
                img.width as i32,
                img.height as i32,
                0,
                img.format,
                gl::UNSIGNED_BYTE,
                img.bytes as *const _,
            );
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
            
        opengl::gl_check_errors();
    }

    pub fn size(&self) -> Vec2 { self.original_size }
    pub fn width(&self) -> f32 { self.size().x }
    pub fn height(&self) -> f32 { self.size().y }
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

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr())
        }
    }
}

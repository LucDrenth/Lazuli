use std::path::Path;

use gl::{types::{GLuint, GLenum}};
use image::{EncodableLayout, RgbaImage};

use crate::{error::opengl, lz_core_warn};

pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub fn new() -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
        }

        opengl::gl_check_errors();
        Self { id }
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

    pub fn load_from_path(&self, path: &Path) {
        self.bind();

        match image::open(path) {
            Ok(img) => {
                Self::upload(&img.into_rgba8());
            },
            Err(err) => {
                lz_core_warn!("Failed to load texture image from path {:?}: {}", path, err);
            },
        }
    }

    pub fn load_from_image(&self, img: &RgbaImage) {
        self.bind();
        Self::upload(img);
    }

    fn upload(img: &RgbaImage) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _,
            );
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
            
        opengl::gl_check_errors();
    }
}

fn to_gl_texture_unit(unit: u32) -> GLenum {
    let lowest = gl::TEXTURE0;
    let highest = gl::TEXTURE31;

    if lowest + unit > highest {
        lz_core_warn!("Texture unit {} is higher than limit {}. Using {} instead.", unit, highest - lowest, highest - lowest);
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

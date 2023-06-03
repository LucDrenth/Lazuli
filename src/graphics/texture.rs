use std::path::Path;

use gl::{types::{GLuint, GLenum, GLfloat}, REPEAT};
use image::EncodableLayout;

use crate::{error::opengl, lz_core_warn};

pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);
        opengl::gl_check_errors();
        Self { id }
    }

    pub unsafe fn activate(&self, unit: usize) {
        gl::ActiveTexture(to_gl_texture_unit(unit as u32));
        self.bind();
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
        opengl::gl_check_errors();
    }

    pub unsafe fn load(&self, path: &Path) {
        self.bind();

        // TODO don't use unwrap and return a Result here
        let img = image::open(path).unwrap().into_rgba8();
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

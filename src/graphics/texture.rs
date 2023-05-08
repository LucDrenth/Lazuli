use std::path::Path;

use gl::types::GLuint;
use image::EncodableLayout;

use crate::error::opengl;

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

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
        opengl::gl_check_errors();
    }

    pub unsafe fn load(&self, path: &Path) {
        self.bind();

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
        gl::GenerateMipmap(gl::TEXTURE_2D);

        opengl::gl_check_errors();

    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr())
        }
    }
}

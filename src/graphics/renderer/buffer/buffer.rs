use gl::types::{GLuint, GLsizeiptr};

use crate::error::opengl;

pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
    pub data_size: i32,
}

impl Buffer {
    pub fn new_vbo() -> Self {
       return Buffer::new(gl::ARRAY_BUFFER)
    }

    pub fn new_ebo() -> Self {
       return Buffer::new(gl::ELEMENT_ARRAY_BUFFER)
    }

    fn new(target: GLuint) -> Self {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        opengl::gl_check_errors();
        Self { 
            id, 
            target,
            data_size: 0 // will be set when setting our data
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }

        opengl::gl_check_errors();
    }

    /// # Arguments
    ///
    /// * `usage` - one of: gl::STREAM_DRAW, gl::STATIC_DRAW, gl::DYNAMIC_DRAW
    pub fn set_data<D>(&mut self, data: &[D], usage: GLuint) {
        self.bind();

        unsafe {
            let (_, data_bytes, _) = data.align_to::<u8>();
            gl::BufferData(
                self.target,
                data_bytes.len() as GLsizeiptr,
                data_bytes.as_ptr() as *const _,
                usage,
            );
        }

        self.data_size = data.len() as i32;
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

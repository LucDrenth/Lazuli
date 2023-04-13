// TODO should we rename to VBO (Vertex buffer object) or will we use this as other buffers?

use gl::types::{GLuint, GLsizeiptr};

pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
    pub data_size: i32,
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id = 0;
        gl::GenBuffers(1, &mut id);
        Self { id, target, data_size: 0 }
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    // usage is one of: gl::STREAM_DRAW, gl::STATIC_DRAW, gl::DYNAMIC_DRAW
    pub unsafe fn set_data<D>(&mut self, data: &[D], usage: GLuint) {
        self.bind();
        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage,
        );

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

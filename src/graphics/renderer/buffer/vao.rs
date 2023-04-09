// vao stands for Vertex array object

use gl::types::{GLuint, GLint};

pub struct Vao {
    pub id: GLuint
}

impl Vao {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    // TODO might want to use other types than f32 (gl::FLOAT) in the future. Make this variable. 
    // Don't forget to update the set_attribute macro (see its TODO comment)
    //
    // V represents the vertex layout
    pub unsafe fn set_attribute<V: Sized>(
        &self, 
        attribute_position: GLuint,
        components: GLint,
        offset: GLint,
    ) {
        self.bind();
        gl::VertexAttribPointer(
            attribute_position,
            components,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<V>() as GLint,
            offset as *const _,
        );
        gl::EnableVertexAttribArray(attribute_position);
    }
}

impl Drop for Vao {
    fn drop (&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $position:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $position,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32, // TODO this may not be f32 in the future
            member_offset,
        )
    }};
}

use std::ffi::CString;
use std::ptr;
use gl::types::{GLuint, GLenum, GLint};

pub const PATH_COLORED_VERT: &str = "./assets/shaders/colored.vert";
pub const PATH_COLORED_FRAG: &str = "./assets/shaders/colored.frag";
pub const PATH_TEXTURED_VERT: &str = "./assets/shaders/textured.vert";
pub const PATH_TEXTURED_FRAG: &str = "./assets/shaders/textured.frag";

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub unsafe fn new(source_code: &str, shader_type: GLenum) -> Result<Self, String> {
        let source_code = CString::new(source_code).unwrap();
        let shader = Self {
            id: gl::CreateShader(shader_type),
        };

        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

        if success == 1 {
            Ok(shader)
        } else {
            Err(shader.get_shader_error())
        }
    }

    unsafe fn get_shader_error(&self) -> String {
        let mut error_log_size: GLint = 0;
        gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
        let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
        gl::GetShaderInfoLog(
            self.id, 
            error_log_size, 
            &mut error_log_size, 
            error_log.as_mut_ptr() as *mut _,
        );

        error_log.set_len(error_log_size as usize);
        let log = String::from_utf8(error_log);
        
        match log {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

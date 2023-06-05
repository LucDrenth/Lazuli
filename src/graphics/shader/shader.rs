use std::{ffi::CString, fs};
use std::ptr;
use gl::types::{GLuint, GLenum, GLint};

use crate::error::opengl;

pub const PATH_COLORED_VERT: &str = "./assets/shaders/colored.vert";
pub const PATH_COLORED_FRAG: &str = "./assets/shaders/colored.frag";
pub const PATH_TEXTURED_VERT: &str = "./assets/shaders/textured.vert";
pub const PATH_TEXTURED_FRAG: &str = "./assets/shaders/textured.frag";
pub const PATH_TEXTURE_MASK_VERT: &str = "./assets/shaders/texture-mask.vert";
pub const PATH_TEXTURE_MASK_FRAG: &str = "./assets/shaders/texture-mask.frag";
pub const PATH_HELLO_TRANFORM_VERT: &str = "./assets/shaders/hello-transform.vert";
pub const PATH_MOVING_TRIANGLE_VERT: &str = "./assets/shaders/moving-triangle.vert";
pub const PATH_MOVING_TRIANGLE_FRAG: &str = "./assets/shaders/moving-triangle.frag";

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(path: &str, shader_type: GLenum) -> Result<Self, String> {
        let source_code = load_shader_source(path)?;

        unsafe {
            let shader = Self {
                id: gl::CreateShader(shader_type),
            };
            opengl::gl_check_errors();

            gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
            opengl::gl_check_errors();
            gl::CompileShader(shader.id);
            opengl::gl_check_errors();

            let mut success: GLint = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
            opengl::gl_check_errors();

            if success == 1 {
                Ok(shader)
            } else {
                Err(format!("failed to compile {} shader {}: [{}]", shader_type_to_string(shader_type), path, shader.get_shader_error()))
            }
        }
    }

    unsafe fn get_shader_error(&self) -> String {
        let mut error_log_size: GLint = 0;
        gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
        opengl::gl_check_errors();
        
        let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
        gl::GetShaderInfoLog(
            self.id, 
            error_log_size, 
            &mut error_log_size, 
            error_log.as_mut_ptr() as *mut _,
        );
        opengl::gl_check_errors();

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

/// TODO cache result for if we reuse the shader source
fn load_shader_source(path: &str) -> Result<CString, String> {
    let source_code = fs::read_to_string(path).map_err(|err| {
        format!("failed to read file [{}]: {}", path, err.to_string())
    })?;

    CString::new(source_code).map_err(|err| {
        format!("failed to creating CString from file [{}]: {}", path, err.to_string())
    })
}

fn shader_type_to_string(shader_type: GLenum) -> String {
    match shader_type {
        gl::VERTEX_SHADER => String::from("vertex"),
        gl::FRAGMENT_SHADER => String::from("fragment"),
        gl::GEOMETRY_SHADER => String::from("geometry"),
        _ => String::from("unknown"),
    }
}

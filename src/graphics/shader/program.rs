use std::ffi::{NulError, CString};

use gl::types::{GLuint, GLint};

use crate::error::opengl;

use super::shader::Shader;

pub struct ShaderProgram {
    pub id: GLuint
}

impl ShaderProgram {
    pub unsafe fn new (shaders: &[Shader]) -> Result<Self, String> {
        let program = Self {
            id: gl::CreateProgram()
        };

        opengl::gl_check_errors();

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        opengl::gl_check_errors();

        gl::LinkProgram(program.id);
        opengl::gl_check_errors();

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        opengl::gl_check_errors();

        if success == 1 {
            Ok(program)
        } else {
            Err(program.get_shader_program_error())
        }
    }

    pub unsafe fn apply (&self) {
        gl::UseProgram(self.id);
        opengl::gl_check_errors();
    }

    pub unsafe fn get_attribute_location(&self, attribute: &str) -> Result<GLuint, NulError> {
        let attribute = CString::new(attribute).expect("Could not create attribute CString");

        let result = gl::GetAttribLocation(self.id, attribute.as_ptr()) as GLuint;
        opengl::gl_check_errors();

        Ok(result)
    }

    unsafe fn get_shader_program_error(&self) -> String {
        let mut error_log_size: GLint = 0;
        gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
        opengl::gl_check_errors();

        let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
        gl::GetProgramInfoLog(
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

    pub unsafe fn set_uniform_int(&self, name: &str, value: i32 ) {
        self.apply();
        let uniform = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);
        opengl::gl_check_errors();
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id)
        }
    }
}

use std::fs;

use crate::{
    graphics::{
        shader::{
            ShaderProgram, 
            Shader, 
            PATH_BASE_VERT, 
            PATH_BASE_FRAG
        }, 
        shapes::{
            Triangle, 
            Shape, 
            Rectangle
        }
    }, 
    error::opengl
};

pub struct Renderer {
    triangle: Triangle,
    rectangle: Rectangle,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let vertex_shader_source = fs::read_to_string(PATH_BASE_VERT).unwrap();
        let fragment_shader_source = fs::read_to_string(PATH_BASE_FRAG).unwrap();

        unsafe {
            // Create shaders
            let vertex_shader = Shader::new(vertex_shader_source.as_str(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(fragment_shader_source.as_str(), gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            let triangle = Triangle::new(program);

            let vertex_shader = Shader::new(vertex_shader_source.as_str(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(fragment_shader_source.as_str(), gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            let rectangle = Rectangle::new(program);
            
            let result = Self { triangle, rectangle };

            opengl::gl_check_errors();

            Ok(result)
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.45, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.rectangle.draw();
            self.triangle.draw();

            opengl::gl_check_errors();
        }
    }
}

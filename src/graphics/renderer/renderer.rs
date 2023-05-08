use std::{fs, path::Path};

use crate::{
    graphics::{
        shader::{
            ShaderProgram, 
            Shader, 
            PATH_COLORED_VERT, 
            PATH_COLORED_FRAG,
            PATH_TEXTURED_VERT,
            PATH_TEXTURED_FRAG,
        }, 
        shapes::{
            Triangle, 
            Shape, 
            Rectangle
        }, texture::Texture
    }, 
    error::opengl
};

pub struct Renderer {
    triangle: Triangle,
    rectangle: Rectangle,
    texture: Texture,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let colored_vertex_shader_source = fs::read_to_string(PATH_COLORED_VERT).unwrap();
        let colored_fragment_shader_source = fs::read_to_string(PATH_COLORED_FRAG).unwrap();

        let textured_vertex_shader_source = fs::read_to_string(PATH_TEXTURED_VERT).unwrap();
        let textured_fragment_shader_source = fs::read_to_string(PATH_TEXTURED_FRAG).unwrap();


        unsafe {
            let vertex_shader = Shader::new(colored_vertex_shader_source.as_str(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(colored_fragment_shader_source.as_str(), gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            let triangle = Triangle::new(program);

            // let vertex_shader = Shader::new(colored_vertex_shader_source.as_str(), gl::VERTEX_SHADER)?;
            // let fragment_shader = Shader::new(colored_fragment_shader_source.as_str(), gl::FRAGMENT_SHADER)?;
            // let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            // let rectangle = Rectangle::new_colored(program);

            let vertex_shader = Shader::new(textured_vertex_shader_source.as_str(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(textured_fragment_shader_source.as_str(), gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            let rectangle = Rectangle::new_textured(program);

            let texture = Texture::new();
            texture.load(&Path::new("./assets/images/lazuli-rock.png"));
            
            let result = Self { triangle, rectangle, texture };

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
            
            opengl::gl_check_errors();

            Ok(result)
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.45, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.triangle.draw();
            self.rectangle.draw();

            opengl::gl_check_errors();
        }
    }
}

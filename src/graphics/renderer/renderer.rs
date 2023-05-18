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
    rectangle_colored: Rectangle,
    rectangle_textured: Rectangle,
    texture0: Texture,
    texture1: Texture,
    program_colored: ShaderProgram,
    program_textured: ShaderProgram,
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
            let program_colored = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            
            let vertex_shader = Shader::new(textured_vertex_shader_source.as_str(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(textured_fragment_shader_source.as_str(), gl::FRAGMENT_SHADER)?;
            let program_textured = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let triangle = Triangle::new(&program_colored);
            let rectangle_colored = Rectangle::new_colored(&program_colored);
            let rectangle_textured = Rectangle::new_textured(&program_textured);

            let texture0 = Texture::new();
            texture0.load(&Path::new("./assets/images/lazuli-rock.png"));
            program_textured.set_uniform_int("texture0", 0);

            let texture1 = Texture::new();
            texture1.load(&Path::new("./assets/images/rust-logo.png"));
            program_textured.set_uniform_int("texture1", 1);
            
            let result = Self { 
                triangle, 
                rectangle_colored, 
                rectangle_textured, 
                texture0,
                texture1,
                program_colored,
                program_textured,
            };

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

            self.triangle.draw(&self.program_colored);

            self.texture0.activate(gl::TEXTURE0);
            self.texture1.activate(gl::TEXTURE1);
            // TODO put this in to a mask struct/image

            self.rectangle_colored.draw(&self.program_colored);
            self.rectangle_textured.draw(&self.program_textured);

            opengl::gl_check_errors();
        }
    }
}

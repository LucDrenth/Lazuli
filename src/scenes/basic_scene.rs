use std::{fs, path::Path};

use crate::graphics::{scene::Scene, material::Material, Triangle, Rectangle, mesh_renderer, shader::{Shader, ShaderProgram, PATH_COLORED_VERT, PATH_COLORED_FRAG, PATH_TEXTURED_VERT, PATH_TEXTURED_FRAG}};

pub struct BasicScene {
    material_textured: Material,
    material_colored: Material,
    triangle: Triangle,
    rectangle: Rectangle,
}

impl BasicScene {
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

            let mut material_textured = Material::new(program_textured);
            material_textured.add_texture(&Path::new("./assets/images/lazuli-rock.png"));
            material_textured.add_texture(&Path::new("./assets/images/rust-logo.png"));

            let material_colored = Material::new(program_colored);

            let triangle = Triangle::new(&material_colored.shader_program);
            let rectangle = Rectangle::new_textured(&material_textured.shader_program);

            let result = Self { 
                material_textured,
                material_colored,
                triangle,
                rectangle, 
            };

            Ok(result)
        }
    }
}

impl Scene for BasicScene {
    unsafe fn draw(&self) {
        mesh_renderer::draw_triangle(&self.triangle, &self.material_colored);
        mesh_renderer::draw_rectangle(&self.rectangle, &self.material_textured);
    }
}

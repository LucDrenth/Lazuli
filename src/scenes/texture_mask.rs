use std::{path::Path};

use crate::graphics::{scene::Scene, material::Material, Rectangle, mesh_renderer, shader::{ShaderProgram, PATH_TEXTURE_MASK_VERT, PATH_TEXTURE_MASK_FRAG}};

pub struct TextureMask {
    material_textured: Material,
    rectangle: Rectangle,
}

impl TextureMask {
    pub fn new() -> Result<Self, String> {
        let program_textured = ShaderProgram::new(PATH_TEXTURE_MASK_VERT, PATH_TEXTURE_MASK_FRAG).unwrap();

        let mut material_textured = Material::new(program_textured);
        material_textured.add_texture(&Path::new("./assets/images/lazuli-rock.png"));
        material_textured.add_texture(&Path::new("./assets/images/rust-logo.png"));

        let rectangle = Rectangle::new_textured(&material_textured.shader_program);

        let result = Self { 
            material_textured,
            rectangle, 
        };

        Ok(result)
    }
}

impl Scene for TextureMask {
    fn update(&mut self) {}

    unsafe fn draw(&self) {
        mesh_renderer::draw_rectangle(&self.rectangle, &self.material_textured);
    }
}

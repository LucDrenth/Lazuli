use std::path::Path;

use crate::{graphics::{scene::Scene, material::Material, mesh_renderer, shader::{ShaderProgram, PATH_TEXTURED_VERT, PATH_TEXTURED_FRAG}, Rectangle}};

pub struct HelloTexture {
    material: Material,
    shape: Rectangle,
}

impl HelloTexture {
    pub fn new() -> Result<Self, String> {
        let program = ShaderProgram::new(PATH_TEXTURED_VERT, PATH_TEXTURED_FRAG).unwrap();
        let mut material = Material::new(program);

        material.add_texture(&Path::new("./assets/images/pattern.png"));

        let shape = Rectangle::new_textured(&material.shader_program);

        let result = Self { 
            material,
            shape,
        };

        Ok(result)
    }
}

impl Scene for HelloTexture {
    fn update(&mut self) {}

    unsafe fn draw(&self) {
        mesh_renderer::draw_rectangle(&self.shape, &self.material);
    }
}

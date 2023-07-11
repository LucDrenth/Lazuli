use std::path::Path;

use glam::Vec2;

use crate::{graphics::{scene::Scene, material::Material, mesh_renderer, shader::{ShaderProgram, PATH_TEXTURED_VERT, PATH_TEXTURED_FRAG}, Rectangle}, event::EventSystem, input::Input};

pub struct HelloTexture {
    material: Material,
    shape: Rectangle,
}

impl Scene for HelloTexture {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2) -> Result<Self, String> {
        let program = ShaderProgram::new(&PATH_TEXTURED_VERT.to_string(), &PATH_TEXTURED_FRAG.to_string())?;
        let mut material = Material::new(program);

        material.add_texture_from_path(&Path::new("./assets/images/pattern.png"));

        let shape = Rectangle::new_textured(&material.shader_program);

        let result = Self { 
            material,
            shape,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input) {}

    unsafe fn draw(&self) {
        mesh_renderer::draw_shape(&self.shape, &self.material);
    }
}

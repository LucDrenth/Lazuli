use std::{path::Path};

use glam::Vec2;

use crate::{graphics::{scene::Scene, material::Material, Rectangle, mesh_renderer, shader::{ShaderProgram, PATH_TEXTURE_MASK_VERT, PATH_TEXTURE_MASK_FRAG}}, event::EventSystem, input::Input};

pub struct TextureMask {
    material_textured: Material,
    rectangle: Rectangle,
}

impl Scene for TextureMask {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2) -> Result<Self, String> {
        let program_textured = ShaderProgram::new(PATH_TEXTURE_MASK_VERT, PATH_TEXTURE_MASK_FRAG).unwrap();

        let mut material_textured = Material::new(program_textured);
        material_textured.add_texture_from_path(&Path::new("./assets/images/lazuli-rock.png"));
        material_textured.add_texture_from_path(&Path::new("./assets/images/rust-logo.png"));

        let rectangle = Rectangle::new_textured(&material_textured.shader_program);

        let result = Self { 
            material_textured,
            rectangle, 
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input) {}

    unsafe fn draw(&self) {
        mesh_renderer::draw_shape(&self.rectangle, &self.material_textured);
    }
}

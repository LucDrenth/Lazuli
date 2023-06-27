use glam::Vec2;

use crate::{graphics::{scene::Scene, material::Material, mesh_renderer, shader::ShaderProgram, font::{Font, BitmapBuilder}, Rectangle, text::Text}, event::EventSystem, input::Input};

pub struct HelloText {
    material: Material,
    font: Font,
    bitmap_rectangle: Rectangle,
    text: Text,
}

impl Scene for HelloText {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2) -> Result<Self, String> 
    {
        let program = ShaderProgram::new("./assets/shaders/text-ui.vert", "./assets/shaders/text-ui.frag").unwrap();
        let mut material = Material::new(program);
        let font = Font::new("./assets/fonts/roboto.ttf".to_string(), 
            BitmapBuilder::new()
            .with_characters("abcdABCD".to_string())
            .with_font_size(100.0)
        )?;
        material.add_texture_from_image(&font.image());

        let bitmap_rectangle = Rectangle::new_textured(&material.shader_program);
        
        let text = Text::new("dAbCdAbCaabb".to_string(), &font, &material.shader_program);
        
        let result = Self { 
            material,
            font,
            bitmap_rectangle,
            text,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _input: &Input) {}

    unsafe fn draw(&self) {
        // mesh_renderer::draw_shape(&self.bitmap_rectangle, &self.material);
        self.text.draw(&self.material);
    }
}

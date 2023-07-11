use glam::Vec2;

use crate::{graphics::{scene::Scene, material::Material, shader::ShaderProgram, font::{Font, SdfBitmapBuilder, PlainBitmapBuilder}, Rectangle, ui::{Text, self, TextBuilder}}, event::{EventSystem, WindowResizeEvent, EventReader}, input::Input};

pub struct HelloText {
    plain_material: Material,
    sdf_material: Material,
    plain_font: Font,
    sdf_font: Font,
    plain_text: Text,
    sdf_text: Text,
    window_resize_listener: EventReader<WindowResizeEvent>,
}

impl Scene for HelloText {
    fn new(event_system: &mut EventSystem, window_size: Vec2) -> Result<Self, String> 
    {
        let window_resize_listener = event_system.register::<WindowResizeEvent>();

        let plain_program = ShaderProgram::new("./assets/shaders/text-ui.vert", "./assets/shaders/text-ui.frag").unwrap();
        let mut plain_material = Material::new(plain_program);
        let plain_font = Font::new("./assets/fonts/roboto.ttf".to_string(), PlainBitmapBuilder::new()
            .with_font_size(50.0)
        )?;
        plain_material.add_texture_from_image(plain_font.image());
        let mut plain_text = Text::new("Welcome to Lazuli engine".to_string(), &plain_font, &plain_material.shader_program, &TextBuilder::new()
            .with_text_size(25.0)
            .with_color((255, 255, 255))
            .with_letter_spacing(0.05)
        );
        plain_text.position.y += 250.0;
        plain_material.shader_program.set_uniform("worldPosition", plain_text.position_for_shader());
        plain_material.shader_program.set_uniform("view", ui::view::for_shader(window_size.x, window_size.y));


        let sdf_program = ShaderProgram::new("./assets/shaders/text-ui.vert", "./assets/shaders/text-ui.frag").unwrap();
        let mut sdf_material = Material::new(sdf_program);
        let sdf_font = Font::new("./assets/fonts/roboto.ttf".to_string(), SdfBitmapBuilder::new()
            .with_font_size(50.0)
            .with_spread(8)
            .with_super_sampling_factor(4)
        )?;
        sdf_material.add_texture_from_image(sdf_font.image());
        let sdf_text = Text::new("Welcome to Lazuli engine".to_string(), &sdf_font, &sdf_material.shader_program, &TextBuilder::new()
            .with_text_size(25.0)
            .with_color((255, 255, 255))
            .with_letter_spacing(0.05)
        );
        sdf_material.shader_program.set_uniform("worldPosition", sdf_text.position_for_shader());
        sdf_material.shader_program.set_uniform("view", ui::view::for_shader(window_size.x, window_size.y));
        
        let result = Self { 
            plain_material,
            sdf_material,
            plain_font,
            sdf_font,
            plain_text,
            sdf_text,
            window_resize_listener,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _input: &Input) {
        for e in self.window_resize_listener.read() {
            self.plain_material.shader_program.set_uniform("view", ui::view::for_shader(e.width as f32, e.height as f32));
            self.sdf_material.shader_program.set_uniform("view", ui::view::for_shader(e.width as f32, e.height as f32));
        }
    }

    unsafe fn draw(&self) {
        self.plain_text.draw(&self.plain_material);
        self.sdf_text.draw(&self.sdf_material);
    }
}

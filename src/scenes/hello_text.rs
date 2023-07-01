use glam::Vec2;

use crate::{graphics::{scene::Scene, material::Material, mesh_renderer, shader::ShaderProgram, font::{Font, BitmapBuilder}, Rectangle, ui::{Text, self, TextBuilder}}, event::{EventSystem, WindowResizeEvent, EventReader}, input::Input};

pub struct HelloText {
    material: Material,
    font: Font,
    bitmap_rectangle: Rectangle,
    text: Text,
    window_resize_listener: EventReader<WindowResizeEvent>,
}

impl Scene for HelloText {
    fn new(event_system: &mut EventSystem, window_size: Vec2) -> Result<Self, String> 
    {
        let window_resize_listener = event_system.register::<WindowResizeEvent>();

        let program = ShaderProgram::new("./assets/shaders/text-ui.vert", "./assets/shaders/text-ui.frag").unwrap();
        let mut material = Material::new(program);
        let font = Font::new("./assets/fonts/roboto.ttf".to_string(), 
            BitmapBuilder::new()
            .with_font_size(200.0)
        )?;

        font.save_bitmap("./assets/fonts/roboto.ttf.bitmap.png".to_string())?;

        material.add_texture_from_image(&font.image());
        
        let bitmap_rectangle = Rectangle::new_textured(&material.shader_program);
        
        let mut text = Text::new("Welcome to Lazuli Engine".to_string(), &font, &material.shader_program, &TextBuilder::new()
            .with_text_size(40.0)
            .with_color((89, 4, 49))
        );
        text.position.y = -200.0;
        text.position.x = 500.0;
        material.shader_program.set_uniform("worldPosition", text.position_for_shader());
        material.shader_program.set_uniform("view", ui::view::for_shader(window_size.x, window_size.y));
        
        let result = Self { 
            material,
            font,
            bitmap_rectangle,
            text,
            window_resize_listener,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _input: &Input) {
        for e in self.window_resize_listener.read() {
            self.material.shader_program.set_uniform("view", ui::view::for_shader(e.width as f32, e.height as f32));
        }
    }

    unsafe fn draw(&self) {
        // self.draw_bitmap();
        self.text.draw(&self.material);
    }
}

impl HelloText {
    fn draw_bitmap(&self) {
        mesh_renderer::draw_shape(&self.bitmap_rectangle, &self.material);
    } 
}
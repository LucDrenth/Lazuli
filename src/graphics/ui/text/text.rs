use glam::Vec2;

use crate::{graphics::{font::Font, Transform, material::Material, ui::{ui_element::UiElement, Interface}}, lz_core_warn};

use super::glyph::Glyph;

pub struct Text {
    text: String,
    glyphs: Vec<Glyph>,
    pub transform: Transform,
    pub letter_spacing: f32,
    pub color: (u8, u8, u8),
    total_width: f32,
    text_size: f32, // font size in pixels
    pub position: Vec2, // position in pixels from the center of the screen
    pub font_id: u16,
}

impl UiElement for Text {
    fn material<'a>(&'a self, interface: &'a Interface) -> Option<&Material> {
        match interface.get_font(self.font_id) {
            Some(font) => {
                Some(&font.material)
            },
            None => None,
        }
    }

    fn draw(&self, material: &Material) {        
        material.activate();
        material.shader_program.set_uniform("color", (
            (self.color.0 as f32 / 255.0),
            (self.color.1 as f32 / 255.0),
            (self.color.2 as f32 / 255.0),
        ));

        material.shader_program.set_uniform("worldPosition", self.position_for_shader());

        for glyph in &self.glyphs {
            glyph.draw();
        }
    }
}

impl Text {
    pub fn new(text: String, font: &Font, font_id: u16, text_builder: &TextBuilder) -> Self {
        let mut glyphs: Vec::<Glyph> = Vec::new();

        let bitmap_spread = (font.bitmap_spread() as f32) * 2.0 / font.line_height() as f32;
        let total_width = Self::get_total_width(&text, &font, text_builder.letter_spacing, bitmap_spread);
        let mut start_x: f32 = 0.0 - total_width / 2.0;

        for character in text.chars() {
            match font.get_bitmap_character(character) {
                Some(bitmap_character) => {
                    // These values range from -window_width to window width and -window_height to window_height.
                    // TODO - why do we need to multiple the x by 4 and y by 2? Could this pixel density for the x2 and y2, but what about the other x2?
                    let glyph_start_x = start_x * text_builder.text_size * 4.0;
                    let glyph_end_x = (start_x + bitmap_character.width) * text_builder.text_size * 4.0;
                    let glyph_start_y = -1.0 * text_builder.text_size * 2.0;
                    let glyph_end_y = 1.0 * text_builder.text_size * 2.0;
                    
                    glyphs.push(Glyph::new(bitmap_character, glyph_start_x, glyph_end_x, glyph_start_y, glyph_end_y, &font.material.shader_program));
                    start_x += bitmap_character.width + text_builder.letter_spacing - bitmap_spread;
                },
                None => {
                    if character == ' ' {
                        // the space character does not have a glyph
                        start_x += font.space_size;
                    } else {
                        lz_core_warn!("font bitmap does not contain character [{}] for text [{}]", character, text);
                    }
                },
            }
        }

        Self { 
            text, 
            glyphs,
            transform: Transform::new(),
            letter_spacing: text_builder.letter_spacing,
            color: text_builder.color,
            total_width,
            text_size: text_builder.text_size,
            position: Vec2::ZERO,
            font_id,
        }
    }

    pub fn position_for_shader(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    /// Calculate the total width of the text, ignoring characters that do not have a glyph
    pub fn get_total_width(text: &String, font: &Font, letter_spacing: f32, spread: f32) -> f32 {
        let mut total_width = 0.0;
        let mut has_glyph_to_render = false;
        
        for character in text.chars() {
            match font.get_bitmap_character(character) {
                Some(bitmap_character) => {
                    has_glyph_to_render = true;
                    total_width += bitmap_character.width + letter_spacing - spread;
                },
                None => {
                    if character == ' ' {
                        has_glyph_to_render = true;
                        total_width += font.space_size;
                    } else {
                        // character will not be rendered
                    }
                },
            }
        }

        if !has_glyph_to_render {
            return 0.0
        }

        return total_width - letter_spacing + spread;
    }
}

pub struct TextBuilder {
    text_size: f32, // size in pixels
    color: (u8, u8, u8),
    letter_spacing: f32,
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder { 
            text_size: 20.0,
            color: (255, 255, 255),
            letter_spacing: 0.04,
        }
    }

    pub fn with_text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    pub fn with_color(mut self, color: (u8, u8, u8)) -> Self {
        self.color = color;
        self
    }

    pub fn with_letter_spacing(mut self, letter_spacing: f32) -> Self {
        self.letter_spacing = letter_spacing;
        self
    }
}

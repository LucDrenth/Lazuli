use crate::{graphics::{font::Font, Transform, material::Material, shader::ShaderProgram}, lz_core_warn};

use super::glyph::Glyph;

pub struct Text {
    text: String,
    glyphs: Vec<Glyph>,
    pub transform: Transform,
    pub letter_spacing: f32,
    pub color: (u8, u8, u8),
    total_width: f32,
    text_size: f32,
}

impl Text {
    pub fn new(text: String, font: &Font, text_size: f32, program: &ShaderProgram) -> Self {
        let mut glyphs: Vec::<Glyph> = Vec::new();

        let letter_spacing = 0.1;

        let total_width = Self::get_total_width(&text, &font, letter_spacing);
        let mut start_x: f32 = 0.0 - total_width / 2.0;

        for character in text.chars() {
            match font.get_bitmap_character(character) {
                Some(bitmap_character) => {
                    let glyph_start_x = start_x * text_size;
                    let glyph_end_x = (start_x + bitmap_character.width) * text_size;
                    let glyph_start_y = -1.0 * text_size;
                    let glyph_end_y = 1.0 * text_size;
                    
                    glyphs.push(Glyph::new(bitmap_character, glyph_start_x, glyph_end_x, glyph_start_y, glyph_end_y, program));
                    start_x += bitmap_character.width + letter_spacing;
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
            letter_spacing,
            color: (255, 0, 0),
            total_width,
            text_size,
        }
    }

    pub fn draw(&self, material: &Material) {
        material.activate();

        // TODO apply program in material.activate and remove program.apply calls in shapes
        material.shader_program.apply();

        for glyph in &self.glyphs {
            glyph.draw();
        }
    }

    /// Calculate the total width of the text, ignoring characters that do not have a glyph
    pub fn get_total_width(text: &String, font: &Font, letter_spacing: f32) -> f32 {
        let mut total_width = 0.0;
        let mut has_glyph_to_render = false;
        
        for character in text.chars() {
            match font.get_bitmap_character(character) {
                Some(bitmap_character) => {
                    has_glyph_to_render = true;
                    total_width += bitmap_character.width + letter_spacing;
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

        return total_width - letter_spacing;
    }
}

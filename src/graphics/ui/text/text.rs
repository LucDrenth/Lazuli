use glam::Vec2;

use crate::{graphics::{font::Font, Transform, ui::ui_element::UiElement}, lz_core_warn, asset_registry::AssetRegistry};

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
    pub z_index: f32, // TODO use this somewhere to determine the render order
    pub font_id: u32,
    material_id: u32,
}

impl UiElement for Text {
    fn draw(&self, asset_registry: &mut AssetRegistry) {   
        asset_registry.activate_material(self.material_id);

        let shader = asset_registry.get_material_shader(self.material_id).unwrap();

        shader.set_uniform("color", (
            (self.color.0 as f32 / 255.0),
            (self.color.1 as f32 / 255.0),
            (self.color.2 as f32 / 255.0),
        ));

        shader.set_uniform("worldPosition", self.position_for_shader());

        for glyph in &self.glyphs {
            glyph.draw();
        }
    }

    fn material_id(&self) -> u32 {
        self.material_id
    }

    fn get_z_index(&self) -> f32 {
        self.z_index
    }
}

impl Text {
    pub fn new(text: String, font_id: u32, text_builder: &TextBuilder, asset_registry: &mut AssetRegistry) -> Result<Self, String> {
        let mut glyphs: Vec::<Glyph> = Vec::new();

        let font_material_id;
        let font_space_size;
        let bitmap_spread;
        let total_width;
        let bitmap_characters;

        match asset_registry.get_font_by_id(font_id) {
            Some(font) => {
                font_material_id = font.material_id;
                font_space_size = font.space_size;
                bitmap_spread = (font.bitmap_spread() as f32) * 2.0 / font.line_height() as f32;
                total_width = Self::get_total_width(&text, &font, text_builder.letter_spacing, bitmap_spread);
                bitmap_characters = font.bitmap_characters_copy()
            },
            None => return Err(format!("Failed to get font by id {}", font_id)),
        }

        let mut start_x: f32 = 0.0 - total_width / 2.0;

        let shader_id = asset_registry.get_material_by_id(font_material_id).unwrap().shader_id;
        let shader = asset_registry.get_shader_by_id(shader_id).unwrap();

        for character in text.chars() {
            match bitmap_characters.get(&character) {
                Some(bitmap_character) => {
                    // These values range from -window_width to window width and -window_height to window_height.
                    // TODO - why do we need to multiple the x by 4 and y by 2? Could this pixel density for the x2 and y2, but what about the other x2?
                    let glyph_start_x = start_x * text_builder.text_size * 4.0;
                    let glyph_end_x = (start_x + bitmap_character.width) * text_builder.text_size * 4.0;
                    let glyph_start_y = -1.0 * text_builder.text_size * 2.0;
                    let glyph_end_y = 1.0 * text_builder.text_size * 2.0;
                    
                    glyphs.push(Glyph::new(bitmap_character, glyph_start_x, glyph_end_x, glyph_start_y, glyph_end_y, shader));
                    start_x += bitmap_character.width + text_builder.letter_spacing - bitmap_spread;
                },
                None => {
                    if character == ' ' {
                        // the space character does not have a glyph
                        start_x += font_space_size;
                    } else {
                        lz_core_warn!("font bitmap does not contain character [{}] for text [{}]", character, text);
                    }
                },
            }
        }

        Ok(Self { 
            text, 
            glyphs,
            transform: Transform::new(),
            letter_spacing: text_builder.letter_spacing,
            color: text_builder.color,
            total_width,
            text_size: text_builder.text_size,
            position: Vec2{x: text_builder.position_x, y: text_builder.position_y},
            z_index: text_builder.z_index,
            font_id,
            material_id: font_material_id,
        })
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

    pub fn total_width(&self) -> f32 {
        self.total_width
    }
}

pub struct TextBuilder {
    text_size: f32, // size in pixels
    color: (u8, u8, u8),
    letter_spacing: f32,
    position_x: f32,
    position_y: f32,
    z_index: f32,
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder { 
            text_size: 20.0,
            color: (255, 255, 255),
            letter_spacing: 0.04,
            position_x: 0.0,
            position_y: 0.0,
            z_index: 1.0,
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

    pub fn with_position_x(mut self, position_x: f32) -> Self {
        self.position_x = position_x;
        self
    }

    pub fn with_position_y(mut self, position_y: f32) -> Self {
        self.position_y = position_y;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        self.z_index = z_index;
        self
    }
}

use glam::Vec2;

use crate::{graphics::{font::Font, Transform, ui::{interface::{is_valid_z_index, map_z_index_for_shader}, Position, element::{world_element_data::WorldElementData, ui_element::UiElement}}, material::Material}, asset_registry::{AssetRegistry, AssetId}, log};

use super::glyph::Glyph;

pub struct Text {
    text: String,
    glyphs: Vec<Glyph>,
    pub transform: Transform,
    pub letter_spacing: f32,
    pub color: (u8, u8, u8),
    font_size: f32, // the font height in pixels
    world_data: WorldElementData,
    pub font_id: AssetId<Font>,
    material_id: AssetId<Material>,
}

impl UiElement for Text {
    fn draw(&self, asset_registry: &mut AssetRegistry) {   
        asset_registry.activate_material(&self.material_id);

        let shader = asset_registry.get_material_shader(&self.material_id).unwrap();

        shader.set_uniform("color", (
            (self.color.0 as f32 / 255.0),
            (self.color.1 as f32 / 255.0),
            (self.color.2 as f32 / 255.0),
        ));
        shader.set_uniform("scale", (self.world_data.scale.x, self.world_data.scale.y));
        shader.set_uniform("zIndex", map_z_index_for_shader(self.world_data.z_index()));
        shader.set_uniform("worldPosition", self.world_data.shader_coordinates());

        for glyph in &self.glyphs {
            glyph.draw();
        }
    }

    fn material_id(&self) -> &AssetId<Material> {
        &self.material_id
    }

    fn type_name(&self) -> &str {
        "text"
    }

    fn world_data(&self) -> &WorldElementData {
        &self.world_data
    }

    fn center_at(&mut self, element_to_center_on: &WorldElementData, window_size: &Vec2) {
        self.world_data.center_at(element_to_center_on, window_size);
    }

    fn handle_window_resize(&mut self, new_window_size: &Vec2) {
        self.world_data.handle_window_resize(new_window_size);
    }

    fn get_scale(&self) -> Vec2 { self.world_data.scale }
    fn set_scale(&mut self, new_scale: Vec2) { self.world_data.scale = new_scale; }
    fn get_size(&self) -> Vec2 { self.world_data.size().clone() }
    fn get_screen_position(&self) -> Vec2 { self.world_data.final_coordinates().clone() }
    
    fn set_text(&mut self, text: &String, asset_registry: &mut AssetRegistry, window_size: &Vec2) -> Result<(), String> {
        // TODO this shares a lot of code with the 'new' funcion
        let font_material_id;
        let font_space_size;
        let bitmap_spread;
        let total_width;
        let bitmap_characters;

        match asset_registry.get_font_by_id(&self.font_id) {
            Some(font) => {
                font_material_id = font.material_id.duplicate();
                font_space_size = font.space_size;
                bitmap_spread = (font.bitmap_spread() as f32) * 2.0 / font.line_height() as f32;
                total_width = Self::get_total_width(&text, &font, self.letter_spacing, bitmap_spread);
                bitmap_characters = font.bitmap_characters_copy()
            },
            None => return Err(format!("Failed to get font by id {}", self.font_id.id())),
        }

        let mut start_x: f32 = 0.0 - total_width / 2.0;
        let worldspace_width = (start_x * self.font_size * 2.0).abs();
        let worldspace_height = self.font_size;

        let shader_id = asset_registry.get_material_by_id(&font_material_id).unwrap().shader_id.duplicate();
        let shader = asset_registry.get_shader_by_id(&shader_id).unwrap();

        self.glyphs = Vec::new();

        for character in text.chars() {
            match bitmap_characters.get(&character) {
                Some(bitmap_character) => {
                    // These values range from (-window_width / 2) to (window_width / 2) and (-window_height / 2) to (window_height / 2).
                    // (0, 0) is the center of the screen.
                    let glyph_start_x = start_x * self.font_size;
                    let glyph_end_x = (start_x + bitmap_character.width) * self.font_size;
                    let glyph_start_y = -1.0 * self.font_size / 2.0;
                    let glyph_end_y = self.font_size / 2.0;
                    
                    self.glyphs.push(Glyph::new(bitmap_character, glyph_start_x, glyph_end_x, glyph_start_y, glyph_end_y, shader));
                    start_x += bitmap_character.width + self.letter_spacing - bitmap_spread;
                },
                None => {
                    if character == ' ' {
                        // the space character does not have a glyph
                        start_x += font_space_size;
                    } else {
                        log::engine_warn(format!("font bitmap does not contain character [{}] for text [{}]", character, text));
                    }
                },
            }
        }

        self.world_data.set_size(Vec2::new(worldspace_width, worldspace_height), window_size);

        Ok(())
    }
}

impl Text {
    pub fn new(text: String, font_id: &AssetId<Font>, text_builder: TextBuilder, asset_registry: &mut AssetRegistry, window_size: &Vec2) -> Result<Self, String> {
        let font_material_id;
        let font_space_size;
        let bitmap_spread;
        let total_width;
        let bitmap_characters;

        match asset_registry.get_font_by_id(&font_id) {
            Some(font) => {
                font_material_id = font.material_id.duplicate();
                font_space_size = font.space_size;
                bitmap_spread = (font.bitmap_spread() as f32) * 2.0 / font.line_height() as f32;
                total_width = Self::get_total_width(&text, &font, text_builder.letter_spacing, bitmap_spread);
                bitmap_characters = font.bitmap_characters_copy()
            },
            None => return Err(format!("Failed to get font by id {}", font_id.id())),
        }

        let mut start_x: f32 = 0.0 - total_width / 2.0;
        let worldspace_width = (start_x * text_builder.font_size * 2.0).abs();
        let worldspace_height = text_builder.font_size;

        let shader_id = asset_registry.get_material_by_id(&font_material_id).unwrap().shader_id.duplicate();
        let shader = asset_registry.get_shader_by_id(&shader_id).unwrap();

        let mut glyphs: Vec::<Glyph> = Vec::new();

        for character in text.chars() {
            match bitmap_characters.get(&character) {
                Some(bitmap_character) => {
                    // These values range from (-window_width / 2) to (window_width / 2) and (-window_height / 2) to (window_height / 2).
                    // (0, 0) is the center of the screen.
                    let glyph_start_x = start_x * text_builder.font_size;
                    let glyph_end_x = (start_x + bitmap_character.width) * text_builder.font_size;
                    let glyph_start_y = -1.0 * text_builder.font_size / 2.0;
                    let glyph_end_y = text_builder.font_size / 2.0;
                    
                    glyphs.push(Glyph::new(bitmap_character, glyph_start_x, glyph_end_x, glyph_start_y, glyph_end_y, shader));
                    start_x += bitmap_character.width + text_builder.letter_spacing - bitmap_spread;
                },
                None => {
                    if character == ' ' {
                        // the space character does not have a glyph
                        start_x += font_space_size;
                    } else {
                        log::engine_warn(format!("font bitmap does not contain character [{}] for text [{}]", character, text));
                    }
                },
            }
        }

        let world_data = WorldElementData::new(
            text_builder.position
            , text_builder.z_index
            , Vec2::new(worldspace_width, worldspace_height)
            , window_size
        );

        Ok(Self { 
            text, 
            glyphs,
            transform: Transform::new(),
            letter_spacing: text_builder.letter_spacing,
            color: text_builder.color,
            font_size: text_builder.font_size,
            world_data,
            font_id: font_id.duplicate(),
            material_id: font_material_id,
        })
    }

    /// Calculate the total width of the text, in local space, ignoring characters that do not have a glyph
    fn get_total_width(text: &String, font: &Font, letter_spacing: f32, spread: f32) -> f32 {
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
    font_size: f32, // size in pixels
    color: (u8, u8, u8),
    letter_spacing: f32,
    position: Position,
    z_index: f32,
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder { 
            font_size: 14.0,
            color: (255, 255, 255),
            letter_spacing: 0.04,
            position: Position::FixedCenter,
            z_index: 10.0,
        }
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
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

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        if is_valid_z_index(z_index) {
            self.z_index = z_index;
        } else {
            log::engine_warn(format!("did not set TextBuilder z_index {} because it's not a valid z-index", z_index));
        }

        self
    }
}

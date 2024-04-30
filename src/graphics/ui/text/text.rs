use glam::{Vec2, Vec4, Vec3};

use crate::{asset_manager::AssetManager, graphics::{font::Font, material::Material, shader::{CustomShaderValues, UniformValue}, ui::{element::{ui_element::UiElement, world_element_data::WorldElementData, AnchorElementData, AnchorPoint, InputEvent}, interface::{self, is_valid_z_index, map_z_index_for_shader}, ElementRegistry, Position}, Color, Transform}, log, ResourceId};

use super::glyph::Glyph;

pub struct Text {
    text: String,
    glyphs: Vec<Glyph>,
    pub transform: Transform,
    pub letter_spacing: f32,
    pub color: Color,
    font_size: f32, // the font height in pixels
    world_data: WorldElementData,
    pub font_id: ResourceId<Box<dyn Font>>,
    material_id: ResourceId<Material>,

    custom_shader_values: CustomShaderValues,
}

impl UiElement for Text {
    fn draw(&self, asset_manager: &mut dyn AssetManager, window_size: &Vec2, pixel_density: f32) {
        if !self.world_data.show {
            return
        }

        match asset_manager.activate_material(&self.material_id){
            Ok(_) => (),
            Err(err) => {
                log::engine_err(format!("Text.draw failed to activate material: {}", err));
                return;
            },
        }

        let shader = asset_manager.get_material_shader(&self.material_id).unwrap();

        shader.set_uniform("color", &UniformValue::from(self.color.to_normalised_rgb_tuple()));
        shader.set_uniform("scale", &UniformValue::from((self.world_data.scale().x, self.world_data.scale().y)));
        shader.set_uniform("zIndex", &UniformValue::from(map_z_index_for_shader(self.world_data.z_index)));
        shader.set_uniform("worldPosition", &UniformValue::from(self.world_data.shader_position()));
        shader.set_uniform("drawBounds", &UniformValue::from(self.world_data.draw_bounds.for_fragment_shader(window_size, pixel_density)));

        self.custom_shader_values.upload(shader);

        for glyph in &self.glyphs {
            glyph.draw();
        }
    }

    fn material_id(&self) -> &ResourceId<Material> {
        &self.material_id
    }

    fn type_name(&self) -> &str {
        "text"
    }

    fn world_data(&self) -> &WorldElementData { &self.world_data }
    fn mut_world_data(&mut self) -> &mut WorldElementData { &mut self.world_data }

    fn mut_custom_shader_values(&mut self) -> &mut CustomShaderValues { &mut self.custom_shader_values }

    fn handle_window_resize(&mut self, new_window_size: &Vec2) {
        self.world_data.handle_window_resize(new_window_size);
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Text {
    pub fn set_text(&mut self, text: &String, window_size: Vec2, anchor_element_data: Option<AnchorElementData>, asset_manager: &mut dyn AssetManager) -> Result<(), String> {
        let font_space_size;
        let bitmap_spread;
        let total_width;
        let bitmap_characters;

        match asset_manager.get_font_by_id(&self.font_id) {
            Some(font) => {
                font_space_size = font.space_size();
                bitmap_spread = (font.bitmap_spread() as f32) * 2.0 / font.line_height() as f32;
                total_width = Self::get_total_width(&text, font, self.letter_spacing, bitmap_spread);
                bitmap_characters = font.atlas().characters().clone()
            },
            None => return Err(format!("Failed to get font by id {}", self.font_id.id())),
        }

        let mut start_x: f32 = 0.0 - total_width / 2.0;
        let worldspace_width = (start_x * self.font_size * 2.0).abs();
        let worldspace_height = self.font_size;

        let shader_id = asset_manager.get_material_by_id(&self.material_id).unwrap().shader_id.duplicate();
        let shader = asset_manager.get_shader_by_id(&shader_id).unwrap();

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

        self.world_data.set_size(Vec2::new(worldspace_width, worldspace_height), window_size, anchor_element_data);

        Ok(())
    }

    /// Calculate the total width of the text, in local space, ignoring characters that do not have a glyph
    fn get_total_width(text: &String, font: &Box<dyn Font>, letter_spacing: f32, spread: f32) -> f32 {
        let mut total_width = 0.0;
        let mut has_glyph_to_render = false;
        
        for character in text.chars() {
            match font.atlas().characters().get(&character) {
                Some(bitmap_character) => {
                    has_glyph_to_render = true;
                    total_width += bitmap_character.width + letter_spacing - spread;
                },
                None => {
                    if character == ' ' {
                        has_glyph_to_render = true;
                        total_width += font.space_size();
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
    color: Color,
    letter_spacing: f32,
    position: Position,
    z_index: f32,
    scale: Vec2,
    is_visible: bool,
    custom_shader_values: CustomShaderValues,
    handle_input_events: Vec<(InputEvent, bool)>,
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder { 
            font_size: interface::default_font_size(),
            color: interface::default_text_color(),
            letter_spacing: 0.04,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            z_index: 10.0,
            scale: Vec2::ONE,
            is_visible: true,
            custom_shader_values: Default::default(),
            handle_input_events: vec![],
        }
    }

    pub fn build(&self, text: impl Into<String>, font_id: &ResourceId<Box<dyn Font>>, asset_manager: &mut dyn AssetManager, element_registry: &mut ElementRegistry) -> Result<Text, String> {
        let font_material_id;
        match asset_manager.get_font_by_id(font_id) {
            Some(font) => {
                font_material_id = font.get_material_id().duplicate();
            },
            None => return Err(format!("Failed to get font by id {}", font_id.id())),
        }

        let mut world_data = WorldElementData::new(
            self.position
            , self.z_index
            , Vec2::new(0.0, 0.0)
            , self.scale
            , element_registry
        );
        world_data.show = self.is_visible;
        world_data.event_handlers.set_handle(false);
        
        for (event, does_handle) in &self.handle_input_events {
            world_data.event_handlers.set_handle_for_input_event(event, *does_handle);
        }

        let mut result = Text { 
            text: text.into(), 
            glyphs: vec![],
            transform: Transform::new(),
            letter_spacing: self.letter_spacing,
            color: self.color.clone(),
            font_size: self.font_size,
            world_data,
            font_id: font_id.duplicate(),
            material_id: font_material_id,
            custom_shader_values: self.custom_shader_values.clone(),
        };
        result.set_text(
            &result.text.clone(), 
            element_registry.size().clone(), 
            result.world_data.position_type().get_anchor_element_data(element_registry), 
            asset_manager
        )?;

        Ok(result)
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
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

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.is_visible = visible;
        self
    }

    pub fn with_custom_shader_value_vec2(mut self, name: impl Into<String>, value: Vec2) -> Self {
        self.custom_shader_values.set_vec2(name, value);
        self
    }
    pub fn with_custom_shader_value_vec3(mut self, name: impl Into<String>, value: Vec3) -> Self {
        self.custom_shader_values.set_vec3(name, value);
        self
    }
    pub fn with_custom_shader_value_vec4(mut self, name: impl Into<String>, value: Vec4) -> Self {
        self.custom_shader_values.set_vec4(name, value);
        self
    }
    pub fn with_custom_shader_value_f32(mut self, name: impl Into<String>, value: f32) -> Self {
        self.custom_shader_values.set_f32(name, value);
        self
    }

    /// Can be called multiple times for different input events
    pub fn with_handle_input_event(mut self, input_event: InputEvent, handle: bool) -> Self {
        self.handle_input_events.push((input_event, handle));
        self
    }
}

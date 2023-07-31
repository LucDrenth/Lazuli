use glam::Vec2;

use crate::{graphics::{ui::{ElementRegistry, interface::{is_valid_z_index, self}, Text, TextBuilder, Position, shapes::{Rectangle, RectangleBuilder}, element::AnchorPoint}, font::PlainBitmapBuilder}, asset_registry::AssetRegistry, log, input::{Input, MouseButton}};

pub struct Slider {
    text_element_id: u32,
    background_element_id: u32,
    progress_element_id: u32,
    value: f32,
    minimum_value: f32,
    maximum_value: f32,
    decimals: usize,
    id: u32,
    scale: Vec2,
    z_index: f32,
}

#[derive(Clone, Copy)]
pub struct SliderUpdateResult {
    pub change_amount: f32,
    pub new_value: f32,
    pub did_start_drag: bool,
}

impl Slider {
    pub fn new(builder: SliderBuilder, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) -> Result<Self, String> {
        let font_id = match builder.font_path {
            Some(font_path) => asset_registry.load_font(PlainBitmapBuilder::new()
                .with_font_file_path(font_path)
                .with_font_size(50.0)
                , None)?,
            None => element_registry.default_font(asset_registry)?,
        };

        let background = Rectangle::new(RectangleBuilder::new()
            .with_width(builder.width)
            .with_height(builder.height)
            .with_z_index(builder.z_index)
            .with_position(builder.position)
            .with_color(builder.background_color)
            .with_scale(builder.scale)
        , asset_registry, element_registry)?;
        let background_element_id = element_registry.add_element(background);

        let progress_rectangle = Rectangle::new(RectangleBuilder::new()
            .with_width(builder.width)
            .with_height(builder.height)
            .with_z_index(builder.z_index + 0.01)
            .with_color(builder.progress_color)
            .with_position(Position::ElementAnchor(AnchorPoint::LeftInside(0.0), background_element_id)) // TODO why does it not center at the left?
            .with_scale(Vec2::new(builder.initial_value / (builder.maximum_value - builder.minimum_value), 1.0) * builder.scale)
        , asset_registry, element_registry)?;
        let progress_element_id = element_registry.add_element(progress_rectangle);

        let text = Text::new(Self::value_string(builder.initial_value, builder.decimals), &font_id, TextBuilder::new()
            .with_color(builder.text_color)
            .with_z_index(builder.z_index + 0.02)
            .with_scale(builder.scale)
            .with_position(Position::ElementAnchor(AnchorPoint::Center, background_element_id))
        , asset_registry, element_registry)?;
        let text_element_id = element_registry.add_element(text);

        Ok(Self {
            text_element_id,
            background_element_id,
            progress_element_id,
            value: builder.initial_value,
            minimum_value: builder.minimum_value,
            maximum_value: builder.maximum_value,
            decimals: builder.decimals,
            id: element_registry.generate_element_id(),
            scale: builder.scale,
            z_index: builder.z_index,
        })
    }

    /// Returns Some if there is a change by dragging the slider
    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) -> Option<SliderUpdateResult> {
        let did_start_drag = self.check_activate_drag(input, element_registry);

        if !element_registry.is_element_dragged(self.id) {
            return None;
        }

        let old_value = self.value;
        self.handle_drag(input, element_registry, asset_registry);

        Some(SliderUpdateResult{
            change_amount: self.value - old_value,
            new_value: self.value,
            did_start_drag,
        })
    }

    /// Check if we should enable dragging
    fn check_activate_drag(&self, input: &Input, element_registry: &mut ElementRegistry) -> bool {
        if input.is_mouse_button_down(MouseButton::Left) && self.is_hovered(input, element_registry) {
            return element_registry.try_set_dragged_element(self.id);
        }

        false
    }

    fn handle_drag(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) {
        let element_size = element_registry.get_element_size(self.background_element_id).unwrap();
        let element_position = element_registry.get_element_screen_position(self.background_element_id).unwrap();

        let element_start_x = element_position.x - element_size.x / 2.0;
        let element_end_x = element_position.x + element_size.x / 2.0;
        let normalised_value = (element_registry.map_mouse_position(input).x - element_start_x) / (element_end_x - element_start_x);

        self.set_normalised_value(normalised_value, element_registry, asset_registry);
    }

    pub fn is_hovered(&self, input: &Input, element_registry: &ElementRegistry) -> bool {
        element_registry.is_element_hovered(self.background_element_id, input)
    }

    pub fn value(&self) -> f32 { self.value }

    pub fn set_value(&mut self, value: f32, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) {
        self.value = value.clamp(self.minimum_value, self.maximum_value);
        self.update_progress_element(element_registry);
        self.update_text_element(element_registry, asset_registry)
    }

    pub fn translate_value(&mut self, extra_value: f32, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) {
        self.set_value(self.value + extra_value, element_registry, asset_registry);
    }

    pub fn set_normalised_value(&mut self, normalised_value: f32, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) {
        let value = (self.maximum_value - self.minimum_value) * normalised_value + self.minimum_value;
        self.set_value(value, element_registry, asset_registry);
    }

    fn update_progress_element(&self, element_registry: &mut ElementRegistry) {
        let scale = Vec2::new(self.value / (self.maximum_value - self.minimum_value), 1.0) * self.scale;
        
        match element_registry.set_element_scale(self.progress_element_id, scale) {
            Ok(_) => (),
            Err(err) => {
                log::engine_err(format!("slider update_progress_element failed: {}", err));
            },
        }
    }

    fn update_text_element(&mut self, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) {
        match element_registry.set_text(self.text_element_id, &Self::value_string(self.value, self.decimals), asset_registry) {
            Ok(_) => (),
            Err(err) => {
                log::warn(format!("failed to update slider text: {}", err));
            },
        }
    }

    fn value_string(value: f32, decimals: usize) -> String {
        format!("{:.decimals$}", value, decimals = decimals)
    }

    pub fn size(&self, element_registry: &ElementRegistry) -> Vec2 {
        element_registry.get_element_size(self.background_element_id).unwrap()
    }

    /// Background is the main element. It defines the position and size of the slider
    pub fn anchor_element_id(&self) -> u32 {
        self.background_element_id
    }

    pub fn set_scale(&mut self, scale: Vec2, element_registry: &mut ElementRegistry) -> Result<(), String> {
        self.scale = scale;

        element_registry.set_element_scale(self.background_element_id, scale)?;
        element_registry.set_element_scale(self.text_element_id, scale)?;
        self.update_progress_element(element_registry);

        Ok(())
    }

    pub fn z_index(&self) -> f32 {
        self.z_index
    }
}

pub struct SliderBuilder {
    background_color: (u8, u8, u8),
    progress_color: (u8, u8, u8),
    text_color: (u8, u8, u8),
    z_index: f32,
    position: Position,
    font_path: Option<String>,
    minimum_value: f32,
    maximum_value: f32,
    initial_value: f32,
    width: f32,
    height: f32,
    decimals: usize,
    scale: Vec2,
}

impl SliderBuilder {
    pub fn new() -> Self {
        Self {
            z_index: 10.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            background_color: interface::default_element_background_color(),
            progress_color: (31, 90, 147),
            text_color: interface::default_text_color(),
            font_path: None,
            minimum_value: 0.0,
            maximum_value: 1.0,
            initial_value: 0.5,
            width: 100.0,
            height: 25.0,
            decimals: 2,
            scale: Vec2::ONE,
        }
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        // Add 1 because the given z_index is for the lowest element, and higher elements may go up to 1 higher
        if is_valid_z_index(z_index + 1.0) {
            self.z_index = z_index;
        } else {
            log::engine_warn(format!("did not set SliderBuilder z_index {} because it's not a valid z-index", z_index));
        }

        self
    }

    pub fn with_text_color(mut self, text_color: (u8, u8, u8)) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn with_font_path(mut self, font_path: String) -> Self {
        self.font_path = Some(font_path);
        self
    }

    pub fn with_minimum_value(mut self, minimum_value: f32) -> Self {
        self.minimum_value = minimum_value;
        self
    }

    pub fn with_maximum_value(mut self, maximum_value: f32) -> Self {
        self.maximum_value = maximum_value;
        self
    }

    pub fn with_initial_value(mut self, initial_value: f32) -> Self {
        self.initial_value = initial_value;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

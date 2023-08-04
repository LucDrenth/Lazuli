use glam::Vec2;

use crate::{graphics::{ui::{Text, TextBuilder, shapes::RectangleBuilder, ElementRegistry, self, interface::{is_valid_z_index, self}, Position, element::{ui_element::UiElement, AnchorPoint}}, font::PlainBitmapBuilder, Color}, asset_manager::AssetManager, input::Input, log};

pub struct Button {
    text_element_id: u32,
    background_element_id: u32,
}

impl Button {
    pub fn new(label: String, builder: ButtonBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<Self, String> {
        let font_id = match builder.font_path {
            Some(font_path) => asset_manager.load_font(PlainBitmapBuilder::new()
                .with_font_file_path(font_path)
                .with_font_size(50.0)
                , None)?,
            None => interface::default_font(asset_manager)?,
        };

        let mut text = Text::new(label, &font_id, TextBuilder::new()
            .with_color(builder.text_color)
            .with_z_index(builder.z_index + 0.01)
            .with_font_size(builder.font_size)
            .with_scale(builder.scale)
        , asset_manager, element_registry)?;
        
        let background_width = text.world_data().width() + builder.padding_x * 2.0;
        let background_height = text.world_data().height() + builder.padding_y * 2.0;

        let background = ui::shapes::Rectangle::new(RectangleBuilder::new()
            .with_width(background_width)
            .with_height(background_height)
            .with_color(builder.background_color)
            .with_z_index(builder.z_index)
            .with_position(builder.position)
            .with_scale(builder.scale)
        , asset_manager, element_registry)?;
        let background_element_id = element_registry.add_rectangle(background);

        text.set_position(Position::ElementAnchor(AnchorPoint::Center, background_element_id), element_registry);
        let text_element_id = element_registry.add_text(text);

        Ok(Self {
            text_element_id,
            background_element_id,
        })
    }

    pub fn is_hovered(&self, input: &Input, element_registry: &ElementRegistry) -> bool {
        element_registry.is_element_hovered(self.background_element_id, input)
    }

    pub fn is_clicked(&self, input: &Input, element_registry: &ElementRegistry) -> bool {
        element_registry.is_element_clicked(self.background_element_id, input)
    }

    pub fn set_scale(&mut self, scale: Vec2, element_registry: &mut ElementRegistry) -> Result<(), String> {
        element_registry.set_element_scale(self.background_element_id, scale)?;
        element_registry.set_element_scale(self.text_element_id, scale)?;
        Ok(())
    }

    /// Background is the main element. It defines the position and size of the slider
    pub fn anchor_element_id(&self) -> u32 {
        self.background_element_id
    }
}

pub struct ButtonBuilder {
    background_color: Color,
    text_color: Color,
    font_path: Option<String>,
    padding_x: f32,
    padding_y: f32,
    z_index: f32,
    position: Position,
    font_size: f32,
    scale: Vec2,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            background_color: interface::default_element_background_color(),
            text_color: interface::default_text_color(),
            font_path: None,
            padding_x: 14.0,
            padding_y: 8.0,
            z_index: 10.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            font_size: interface::default_font_size(),
            scale: Vec2::ONE,
        }
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn with_text_color(mut self, text_color: Color) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn with_font_path(mut self, font_path: String) -> Self {
        self.font_path = Some(font_path);
        self
    }

    pub fn with_padding_x(mut self, padding_x: f32) -> Self {
        self.padding_x = padding_x;
        self
    }

    pub fn with_padding_y(mut self, padding_y: f32) -> Self {
        self.padding_x = padding_y;
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding_x = padding;
        self.padding_y = padding;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        // Add 1 because the given z_index is for the lowest element, and higher elements may go up to 1 higher
        if is_valid_z_index(z_index + 1.0) {
            self.z_index = z_index;
        } else {
            log::engine_warn(format!("Did not set ButtonBuilder z_index {} because it's not a valid z-index", z_index));
        }

        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

use crate::{graphics::{ui::{Text, TextBuilder, shapes::RectangleBuilder, Interface, self, interface::is_valid_z_index, ui_element::UiElement, Position}, font::PlainBitmapBuilder}, asset_registry::AssetRegistry, lz_core_warn, input::Input};

pub struct Button {
    text_element_id: u32,
    background_element_id: u32,
}

impl Button {
    pub fn new(label: String, builder: ButtonBuilder, interface: &mut Interface, asset_registry: &mut AssetRegistry) -> Result<Self, String> {
        let font_id = asset_registry.load_font(PlainBitmapBuilder::new()
            .with_font_file_path(builder.font_path)
            .with_font_size(50.0)
        , None)?;

        let text = Text::new(label, font_id, TextBuilder::new()
            .with_color(builder.text_color)
            .with_z_index(builder.z_index + 0.01)
            .with_position(builder.position)
            .with_font_size(builder.font_size)
        , asset_registry, interface.size())?;
        
        let background_width = text.world_data().width() + builder.padding_x * 2.0;
        let background_height = text.world_data().height() + builder.padding_y * 2.0;

        let background = ui::shapes::Rectangle::new(RectangleBuilder::new()
            .with_width(background_width)
            .with_height(background_height)
            .with_color(builder.background_color)
            .with_z_index(builder.z_index)
            .with_position(builder.position)
        , asset_registry, interface.size())?;

        let background_element_id = interface.add_element(background);
        let text_element_id = interface.add_element(text);

        Ok(Self {
            text_element_id,
            background_element_id,
        })
    }

    pub fn is_hovered(&self, input: &Input, interface: &Interface) -> bool {
        interface.is_element_hovered(self.background_element_id, input)
    }

    pub fn is_clicked(&self, input: &Input, interface: &Interface) -> bool {
        interface.is_element_clicked(self.background_element_id, input)
    }
}

pub struct ButtonBuilder {
    background_color: (u8, u8, u8),
    text_color: (u8, u8, u8),
    font_path: String,
    padding_x: f32,
    padding_y: f32,
    z_index: f32,
    position: Position,
    font_size: f32,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            background_color: (126, 126, 126),
            text_color: (255, 255, 255),
            font_path: "./assets/fonts/roboto.ttf".to_string(),
            padding_x: 14.0,
            padding_y: 8.0,
            z_index: 10.0,
            position: Position::FixedCenter,
            font_size: 14.0,
        }
    }

    pub fn with_background_color(mut self, background_color: (u8, u8, u8)) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn with_text_color(mut self, text_color: (u8, u8, u8)) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn with_font_path(mut self, font_path: String) -> Self {
        self.font_path = font_path;
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
            lz_core_warn!("did not set ButtonBuilder z_index {} because it's not a valid z-index", z_index);
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
}

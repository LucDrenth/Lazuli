use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{self, shapes::RectangleBuilder, Button, ButtonBuilder, TextBuilder}, font::PlainBitmapBuilder}, event::EventSystem, input::{Input, Key}, asset_registry::AssetRegistry, lz_core_info};

pub struct HelloUi {
    interface: ui::Interface,
    button: Button,
    text_id: u32,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system, window_size);

        let button = Button::new("Click me!".to_string(), ButtonBuilder::new()
            .with_position(ui::Position::FixedLeft(25.0))
            .with_padding(16.0)
        , &mut interface, asset_registry)?;

        let plain_font_id = asset_registry.load_font(PlainBitmapBuilder::new()
            .with_font_size(50.0)
        , None)?;
        let text_id = interface.add_text("I am a text you can click".to_string(), plain_font_id, TextBuilder::new()
            .with_position(ui::Position::FixedTop(25.0))
        , asset_registry)?;

        Ok(Self { 
            interface,
            button,
            text_id,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_registry: &mut AssetRegistry) {
        self.interface.update(asset_registry);

        if input.is_key_down(Key::Space) {
            self.interface.add_rectangle(RectangleBuilder::new()
                .with_color((255, 25, 162))
                .with_width(self.interface.width() * 0.95)
                .with_height(self.interface.height() * 0.95)
                .with_z_index(5.0), asset_registry
            ).unwrap();
        }

        if self.button.is_clicked(input, &self.interface) {
            lz_core_info!("Button was clicked!");
        }

        if self.interface.is_element_clicked(self.text_id, input) {
            lz_core_info!("Text was clicked!");
        }
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

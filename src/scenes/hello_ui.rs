use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{self, shapes::RectangleBuilder, TextBuilder, widget::{Button, ButtonBuilder}}, font::PlainBitmapBuilder}, event::EventSystem, input::{Input, Key}, asset_registry::AssetRegistry, log};

pub struct HelloUi {
    interface: ui::Interface,
    button: Button,
    text_id: u32,
    rectangle_id: u32,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system, window_size);

        let button = Button::new("Click me!".to_string(), ButtonBuilder::new(&interface)
            .with_position(ui::Position::FixedBottomRight(25.0, 10.0))
            .with_padding(16.0)
        , &mut interface, asset_registry)?;

        let plain_font_id = asset_registry.load_font(PlainBitmapBuilder::new()
            .with_font_size(50.0)
        , None)?;
        let text_id = interface.add_text("I am a text you can click".to_string(), &plain_font_id, TextBuilder::new()
            .with_position(ui::Position::FixedTop(25.0))
            .with_z_index(100.0)
        , asset_registry)?;

        let rectangle_id = interface.add_rectangle(RectangleBuilder::new()
            .with_width(400.0)
            .with_height(200.0)
            .with_z_index(50.0)
            .with_color((125, 23, 1))
            .with_position(ui::Position::FixedTopLeft(25.0, 100.0))
        , asset_registry)?;

        Ok(Self { 
            interface,
            button,
            text_id,
            rectangle_id,
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

        if input.is_key_down(Key::C) {
            self.interface.center_element_at_element(self.text_id, self.rectangle_id);
        }

        if self.button.is_clicked(input, &self.interface) {
            log::engine_info(format!("Button was clicked!"));
        }

        if self.interface.is_element_clicked(self.text_id, input) {
            log::engine_info(format!("Text was clicked!"));
        }
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

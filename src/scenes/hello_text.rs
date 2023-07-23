use glam::Vec2;

use crate::{graphics::{scene::Scene, font::{SdfBitmapBuilder, PlainBitmapBuilder}, ui::{Text, self, TextBuilder}}, event::EventSystem, input::Input, asset_registry::AssetRegistry};

pub struct HelloText {
    interface: ui::Interface,
}

impl Scene for HelloText {
    fn new(event_system: &mut EventSystem, _window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system);


        let plain_font_id = asset_registry.load_font(PlainBitmapBuilder::new()
            .with_font_size(50.0)
        , None)?;

        let sdf_font_id = asset_registry.load_font(SdfBitmapBuilder::new()
            .with_font_size(50.0)
            .with_spread(8)
            .with_super_sampling_factor(4)
        , None)?;
        

        // First way of creating a text is with a funciton from Interface
        interface.add_text("Welcome to Lazuli engine".to_string(), plain_font_id, &TextBuilder::new()
            .with_text_size(25.0)
            .with_color((255, 255, 255))
            .with_letter_spacing(0.05)
            .with_position_y(100.0)
        , asset_registry)?;

        // Second way of creating a text is creating one ourselves and adding it to the Interface
        let sdf_text = Text::new("Welcome to Lazuli engine".to_string(), sdf_font_id, &TextBuilder::new()
            .with_text_size(25.0)
            .with_color((255, 255, 255))
            .with_letter_spacing(0.05)
            .with_position_y(-100.0)
        , asset_registry)?;
        interface.add_element(sdf_text);
        

        let result = Self { 
            interface,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _input: &Input, asset_registry: &mut AssetRegistry) {
        self.interface.update(asset_registry);
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{self, shapes::RectangleBuilder, TextBuilder, widget::{Button, ButtonBuilder, Slider, SliderBuilder}, Position}, font::PlainBitmapBuilder}, event::EventSystem, input::{Input, Key}, asset_registry::AssetRegistry, log, time::DELTA};

pub struct HelloUi {
    interface: ui::Interface,
    slider: Slider,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system, window_size);

        let slider = Slider::new(SliderBuilder::new(&interface)
            .with_z_index(500.0)
            .with_position(Position::FixedBottom(20.0))
        , &mut interface, asset_registry)?;

        Ok(Self { 
            interface,
            slider,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_registry: &mut AssetRegistry) {
        self.interface.update(asset_registry);
        self.slider.update(input, &mut self.interface, asset_registry);

        if input.is_key_held(Key::ArrowLeft) {
            self.slider.translate_value(-50.0 * DELTA, &mut self.interface, asset_registry);
        }
        if input.is_key_held(Key::ArrowRight) {
            self.slider.translate_value(50.0 * DELTA, &mut self.interface, asset_registry);
        }
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

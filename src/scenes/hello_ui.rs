use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{self, shapes::{Rectangle, RectangleBuilder}, Button, ButtonBuilder}}, event::EventSystem, input::{Input, Key}, asset_registry::AssetRegistry};

pub struct HelloUi {
    interface: ui::Interface,
    button: Button,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, _window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system);

        let button = Button::new("Click me!".to_string(), ButtonBuilder::new(), &mut interface, asset_registry)?;

        Ok(Self { 
            interface,
            button,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_registry: &mut AssetRegistry) {
        self.interface.update(asset_registry);

        if input.is_key_down(Key::Space) {
            let rectangle = Rectangle::new(RectangleBuilder::new()
                .with_color((255, 25, 162))
                .with_z_index(100.0)
            , asset_registry).unwrap();
            self.interface.add_element(rectangle);
        }
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{self, shapes::{Rectangle, RectangleBuilder}}}, event::EventSystem, input::Input, asset_registry::AssetRegistry};

pub struct HelloUi {
    interface: ui::Interface,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, _window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system);

        let rectangle = Box::new(Rectangle::new(RectangleBuilder::new()
            .with_color((255, 25, 162))
        , asset_registry)?);
        interface.add_element(rectangle);

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

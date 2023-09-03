use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{Interface, UiWidgetId, widget::IconBuilder, TextBuilder, Position}, Color}, event::EventSystem, input::Input, asset_manager::AssetManager, ResourceId};

pub struct HelloIcon {
    interface: Interface,
    icon_id: ResourceId<UiWidgetId>,
}

impl Scene for HelloIcon {
    fn new(event_system: &mut EventSystem, window_size: Vec2, pixel_density: f32, asset_manager: &mut AssetManager) -> Result<Self, String> {
        let mut interface = Interface::new(event_system, window_size, pixel_density);

        let icon_id = interface.add_icon(&IconBuilder::new()
            .with_background_color(Color::green())
        , asset_manager)?;

        interface.mut_element_registry().create_text("Hello icon", None, &TextBuilder::new()
            .with_position(Position::Fixed(-50., 0.))
            .with_z_index(5.)
        , asset_manager)?;

        Ok(Self { 
            interface, 
            icon_id, 
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_manager: &mut AssetManager) {
        self.interface.update(asset_manager, input);
    }

    unsafe fn draw(&self, asset_manager: &mut AssetManager) {
        self.interface.draw(asset_manager);
    }
}

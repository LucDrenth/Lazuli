use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{Interface, UiWidgetId, widget::IconBuilder, TextBuilder, Position}, Color}, event::EventSystem, input::Input, asset_manager::AssetManager, ResourceId};

pub struct HelloIcon {
    icon_id: ResourceId<UiWidgetId>,
}

impl Scene for HelloIcon {
    fn new(_: &mut EventSystem, _: Vec2, _: f32, asset_manager: &mut AssetManager, interface: &mut Interface) -> Result<Self, String> {
        let icon_id = interface.add_icon(&IconBuilder::new()
            .with_background_color(Color::green())
        , asset_manager)?;

        interface.mut_element_registry().create_text("Hello icon", None, &TextBuilder::new()
            .with_position(Position::Fixed(-50., 0.))
            .with_z_index(5.)
        , asset_manager)?;

        Ok(Self { icon_id })
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetManager, _: &mut Interface) {}

    unsafe fn draw(&self, _: &mut AssetManager) {}
}

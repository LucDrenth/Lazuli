use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{Interface, UiWidgetId, widget::IconBuilder, TextBuilder, Position, shapes::RectangleBuilder, AnchorPoint, UiTexture, Padding}, Color}, event::EventSystem, input::Input, asset_manager::AssetManager, ResourceId};

pub struct HelloIcon {
    icon_id: ResourceId<UiWidgetId>,
}

impl Scene for HelloIcon {
    fn new(_: &mut EventSystem, _: Vec2, _: f32, asset_manager: &mut AssetManager, interface: &mut Interface) -> Result<Self, String> {
        let icon_id = interface.add_icon(&IconBuilder::new()
            .with_background_color(Color::green())
        , asset_manager)?;

        _ = interface.mut_element_registry().create_rectangle(&&RectangleBuilder::new()
            .with_color(Color::orange())
            .with_position(Position::ScreenAnchor(AnchorPoint::TopLeftInside(10., 10.)))
            .with_width(100.0)
            .with_height(100.0)
            .with_texture(UiTexture::path("./assets/images/pattern-inner.png"))
            .with_texture_padding(Padding::Universal(25.0))
        , asset_manager);

        interface.mut_element_registry().create_text("Hello icon", None, &TextBuilder::new()
            .with_position(Position::Fixed(-50., 0.))
            .with_z_index(5.)
        , asset_manager)?;

        Ok(Self { icon_id })
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetManager, _: &mut Interface) {}

    unsafe fn draw(&self, _: &mut AssetManager) {}
}

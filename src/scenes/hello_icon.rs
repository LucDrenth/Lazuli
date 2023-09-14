use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{Interface, UiWidgetId, widget::{IconBuilder, SliderBuilder}, TextBuilder, Position, AnchorPoint}, Color}, event::EventSystem, input::Input, asset_manager::AssetManager, ResourceId};

pub struct HelloIcon {
    icon_id: ResourceId<UiWidgetId>,
    tp_slider: ResourceId<UiWidgetId>,
}

impl Scene for HelloIcon {
    fn new(_: &mut EventSystem, _: Vec2, _: f32, asset_manager: &mut AssetManager, interface: &mut Interface) -> Result<Self, String> {
        let padding = 3.0;

        let icon_id = interface.add_icon(&IconBuilder::new()
            .with_background_color(Color::green())
            .with_padding(padding)
        , asset_manager)?;

        interface.mut_element_registry().create_text("Hello icon", None, &TextBuilder::new()
            .with_position(Position::Fixed(-50., 0.))
            .with_z_index(5.)
        , asset_manager)?;

        let tp_slider = interface.add_slider(&SliderBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::BottomOutside(10.0), interface.get_widget_main_element_id(&icon_id).unwrap()))
            .with_initial_value(padding)
            .with_maximum_value(15.0)
        , asset_manager)?;

        Ok(Self { icon_id, tp_slider })
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetManager, interface: &mut Interface) {
        interface.slider_update_result(&self.tp_slider).map(|e| {
            interface.set_icon_padding(e.new_value, &self.icon_id)
        });
    }

    unsafe fn draw(&self, _: &mut AssetManager) {}
}

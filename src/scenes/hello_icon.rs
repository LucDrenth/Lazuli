use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{Interface, shapes::{RectangleBuilder, RectangleTexture}, UiElementId}, Color}, event::EventSystem, input::Input, asset_manager::AssetManager, ResourceId};

pub struct HelloIcon {
    interface: Interface,
    rectangle_id: ResourceId<UiElementId>,
}

impl Scene for HelloIcon {
    fn new(event_system: &mut EventSystem, window_size: Vec2, pixel_density: f32, asset_manager: &mut AssetManager) -> Result<Self, String> {
        let mut interface = Interface::new(event_system, window_size, pixel_density);

        let rectangle_id = interface.mut_element_registry().create_rectangle(&RectangleBuilder::new()
            .without_color()
            .with_color(Color::Rgba(255, 255, 0, 0.75))
            .with_texture(RectangleTexture::Path("./assets/images/lazuli-rock.png".to_string()))
            .with_width(700.0)
            .with_height(500.0)
            .with_border_size(10.0)
            .with_border_color(Color::rgba_red())
        , asset_manager)?;

        Ok(Self { 
            interface, 
            rectangle_id, 
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_manager: &mut AssetManager) {
        self.interface.update(asset_manager, input);
    }

    unsafe fn draw(&self, asset_manager: &mut AssetManager) {
        self.interface.draw(asset_manager);
    }
}

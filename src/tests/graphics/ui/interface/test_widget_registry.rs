use glam::Vec2;

use crate::{asset_manager::AssetManager, event::EventSystem, graphics::ui::{widget::ButtonBuilder, Interface}};

#[test]
fn test_get_button_as_widget() -> Result<(), String> {
    let mut interface = create_interface();
    let asset_manager = &mut AssetManager::new();
    let button_id = interface.create_button("my button", &ButtonBuilder::new(), asset_manager)?;
    // interface.g
    Ok(())
}

fn create_interface() -> Interface {
    let event_system = &mut EventSystem::new();
    let window_size = Vec2::new(1000.0, 750.0);
    Interface::new(event_system, window_size, 1.0)
}

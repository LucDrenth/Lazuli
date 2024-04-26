use glam::Vec2;

use crate::{asset_manager::mock_asset_manager::MockAssetManager, graphics::{ui::widget::ButtonBuilder, Color}};

use super::{ElementRegistry, WidgetRegistry};

#[test]
fn test_button_widget() -> Result<(), String> {
    let mut element_registry = ElementRegistry::new(Vec2{x: 1280.0, y: 780.0}, 1.0);
    let mut widget_registry = WidgetRegistry::new();
    let asset_manager = &mut MockAssetManager::new();
    
    let button_id = widget_registry.create_button("benjamin button", &ButtonBuilder::new(), &mut element_registry, asset_manager)?;

    assert!(widget_registry.get_button(&button_id).is_some());
    assert!(widget_registry.get_widget_by_id(&button_id).is_some());

    widget_registry.set_button_background_color(Color::orange(), &button_id, &mut element_registry)?;
    widget_registry.set_button_text_color(Color::green(), &button_id, &mut element_registry)?;

    Ok(())
}

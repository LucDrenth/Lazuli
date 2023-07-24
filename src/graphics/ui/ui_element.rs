use glam::Vec2;

use crate::asset_registry::AssetRegistry;

use super::world_element_data::WorldElementData;

pub trait UiElement {
    fn material_id(&self) -> u32;
    fn draw(&self, asset_registry: &mut AssetRegistry);
    fn type_name(&self) -> &str;
    fn world_data(&self) -> &WorldElementData;
    fn handle_window_resize(&mut self, new_window_size: &Vec2);
}

use glam::Vec2;

use crate::{asset_registry::{AssetRegistry, AssetId}, graphics::material::Material};

use super::world_element_data::WorldElementData;

pub trait UiElement {
    fn material_id(&self) -> &AssetId<Material>;
    fn draw(&self, asset_registry: &mut AssetRegistry);
    fn type_name(&self) -> &str;
    fn world_data(&self) -> &WorldElementData;
    fn center_at(&mut self, element_to_center_on: &WorldElementData);
    fn handle_window_resize(&mut self, new_window_size: &Vec2);
}

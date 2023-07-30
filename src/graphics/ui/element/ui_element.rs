use glam::Vec2;

use crate::{asset_registry::{AssetRegistry, AssetId}, graphics::material::Material};

use super::{world_element_data::WorldElementData, AnchorElementData};

pub trait UiElement {
    fn material_id(&self) -> &AssetId<Material>;
    fn draw(&self, asset_registry: &mut AssetRegistry);
    fn type_name(&self) -> &str;
    fn handle_window_resize(&mut self, new_window_size: &Vec2);

    fn world_data(&self) -> &WorldElementData;
    fn center_at(&mut self, element_to_center_on: &WorldElementData, window_size: &Vec2);
    fn get_scale(&self) -> Vec2;
    fn set_scale(&mut self, new_scale: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>);
    fn get_size(&self) -> Vec2;
    fn get_screen_position(&self) -> Vec2;

    // TODO these are element specific, but need to be accessed from interface it's ui_element collection
    fn set_text(&mut self, text: &String, asset_registry: &mut AssetRegistry, window_size: &Vec2) -> Result<(), String>;
}

use glam::Vec2;

use crate::{asset_manager::{AssetManager, AssetId}, graphics::{material::Material, ui::ElementRegistry}};

use super::{world_element_data::WorldElementData, AnchorElementData, Position};

pub trait UiElement {
    fn material_id(&self) -> &AssetId<Material>;
    fn draw(&self, asset_manager: &mut AssetManager);
    fn type_name(&self) -> &str;
    fn handle_window_resize(&mut self, new_window_size: &Vec2);

    fn world_data(&self) -> &WorldElementData;
    fn get_scale(&self) -> Vec2;
    fn set_scale(&mut self, new_scale: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>);
    fn get_size(&self) -> Vec2;
    fn get_screen_position(&self) -> Vec2;
    fn set_position(&mut self, position: Position, element_registry: &ElementRegistry);
}

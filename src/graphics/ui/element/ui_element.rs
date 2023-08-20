use glam::Vec2;

use crate::{asset_manager::{AssetManager, AssetId}, graphics::{material::Material, Color}};

use super::world_element_data::WorldElementData;

pub trait UiElement {
    fn material_id(&self) -> &AssetId<Material>;
    fn draw(&self, asset_manager: &mut AssetManager, window_size: &Vec2, pixel_density: f32);
    fn type_name(&self) -> &str;
    fn handle_window_resize(&mut self, new_window_size: &Vec2);

    fn world_data(&self) -> &WorldElementData;
    fn mut_world_data(&mut self) -> &mut WorldElementData;
    
    fn set_color(&mut self, color: Color);
}

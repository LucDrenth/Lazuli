use glam::Vec2;

use crate::{asset_manager::{AssetManager, AssetId}, graphics::{material::Material, Color, ui::draw_bounds::DrawBounds}};

use super::{world_element_data::WorldElementData, AnchorElementData, Position};

pub trait UiElement {
    fn material_id(&self) -> &AssetId<Material>;
    fn draw(&self, asset_manager: &mut AssetManager, window_size: &Vec2, pixel_density: f32);
    fn type_name(&self) -> &str;
    fn handle_window_resize(&mut self, new_window_size: &Vec2);

    fn world_data(&self) -> &WorldElementData;
    fn get_scale(&self) -> Vec2;
    fn set_scale(&mut self, new_scale: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>);
    fn get_size(&self) -> Vec2;
    fn get_screen_position(&self) -> Vec2;
    fn set_position(&mut self, position: Position, window_size: Vec2, anchor_element_data: Option<AnchorElementData>);
    fn recalculate_position(&mut self, window_size: Vec2, anchor_element_data: Option<AnchorElementData>);
    fn set_color(&mut self, color: Color);
    fn set_z_index(&mut self, z_index: f32);
    fn draw_bounds(&self) -> &DrawBounds;
    fn set_draw_bounds(&mut self, draw_bounds: DrawBounds);
    fn set_position_transform(&mut self, position_transform: Vec2);

    fn hide(&mut self);
    fn show(&mut self);
    fn is_shown(&self) -> bool;
}

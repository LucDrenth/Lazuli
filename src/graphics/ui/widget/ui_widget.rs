use glam::Vec2;

use crate::graphics::ui::{ElementRegistry, Position, draw_bounds::DrawBounds};

pub trait UiWidget {
    fn show(&self, element_registry: &mut ElementRegistry);
    fn hide(&self, element_registry: &mut ElementRegistry);
    fn anchor_element_id(&self) -> u32;
    fn z_index(&self) -> f32;
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry);
    fn size(&self, element_registry: &ElementRegistry) -> Result<Vec2, String>;
    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry);
    fn set_draw_bounds(&self, draw_bounds: DrawBounds, element_registry: &mut ElementRegistry);
}

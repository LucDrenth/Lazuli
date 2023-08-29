use glam::Vec2;

use crate::{graphics::ui::{ElementRegistry, Position, bounds_2d::Bounds2d, UiElementId}, ResourceId};

pub trait UiWidget {
    fn show(&self, element_registry: &mut ElementRegistry);
    fn hide(&self, element_registry: &mut ElementRegistry);

    /// Get the id of the main element (usually the background), which can be 
    /// used for positioning, anchoring etcetera. It is usually the background element.
    fn get_main_element_id(&self) -> ResourceId<UiElementId>;

    fn z_index(&self) -> f32;
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry);
    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry);
    fn set_draw_bounds(&self, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry);

    fn set_width(&self, width: f32, element_registry: &mut ElementRegistry);
    fn set_height(&self, height: f32, element_registry: &mut ElementRegistry);
    fn set_size(&self, size: Vec2, element_registry: &mut ElementRegistry);
}

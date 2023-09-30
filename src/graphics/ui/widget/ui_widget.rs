use glam::Vec2;

use crate::{graphics::ui::{ElementRegistry, Position, bounds_2d::Bounds2d, UiElementId, interface::WidgetRegistry}, ResourceId};

use super::ui_update_target::UiUpdateTargets;

pub trait UiWidget {
    fn on_show(&mut self);
    fn on_hide(&mut self);

    fn get_all_element_ids(&self, widget_registry: &WidgetRegistry) -> Vec<ResourceId<UiElementId>>;

    /// Get the id of the main element (usually the background), which can be 
    /// used for positioning, anchoring etcetera. It is usually the background element.
    fn get_main_element_id(&self, widget_registry: &WidgetRegistry) -> ResourceId<UiElementId>;

    fn z_index(&self) -> f32;
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Position>;
    fn set_draw_bounds(&self, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d>;

    fn set_width(&self, width: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_height(&self, height: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_size(&self, size: Vec2, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2>;
}

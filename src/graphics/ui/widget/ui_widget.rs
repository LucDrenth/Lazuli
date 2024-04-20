use glam::Vec2;

use crate::{graphics::ui::{ElementRegistry, Position, bounds_2d::Bounds2d, UiElementId, interface::WidgetRegistry, UiUpdateTargets, UpdateTargetCollection, UiLayoutId, UiWidgetId}, ResourceId};

pub trait UiWidget {
    /// Get direct elements, not including elements of nested widgets and layouts
    fn get_direct_element_ids(&self) -> Vec<ResourceId<UiElementId>>;
    /// Get direct layouts, not including layouts of nested widgets and layouts
    fn get_direct_layout_ids(&self) -> Vec<ResourceId<UiLayoutId>>;
    /// Get direct widgets, not including widgets of nested widgets and layouts
    fn get_direct_widget_ids(&self) -> Vec<ResourceId<UiWidgetId>>;

    /// Get the id of the main element (usually the background), which can be 
    /// used for positioning, anchoring etcetera. It is usually the background element.
    fn get_main_element_id(&self, widget_registry: &WidgetRegistry) -> ResourceId<UiElementId>;

    fn z_index(&self) -> f32;
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry) -> UpdateTargetCollection;
    fn set_draw_bounds(&self, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d>;

    fn set_width(&self, width: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_height(&self, height: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_size(&self, size: Vec2, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2>;
    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool>;

    /// We can mark elements as debug in the widget builders. This does nothing on itself, but can be useful for tracking
    /// and debugging the element
    fn is_debug(&self) -> bool;
}

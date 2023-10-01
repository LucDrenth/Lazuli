use crate::{graphics::ui::{UiWidgetId, ElementRegistry, interface::WidgetRegistry, UiUpdateTargets, UpdateTargetCollection, Position}, input::Input, ResourceId, asset_manager::AssetManager};

/// Widgets in a layout should get a higher z_index than the background
pub const LAYOUT_ELEMENT_EXTRA_Z_INDEX: f32  = 0.1;

pub trait Layout {
    fn add_widget(&mut self, widget_id: &ResourceId<UiWidgetId>, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry);
    fn update(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, input: &Input, scroll_speed: f32);
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool>;
    fn set_position(&mut self, position: Position, element_registry: &mut ElementRegistry) -> UpdateTargetCollection;
    /// The width parameter is used as fixed: `Width::Fixed(width)`
    fn set_width(&mut self, width: f32, element_registry: &mut ElementRegistry) -> UpdateTargetCollection;

    /// Update elements their draw bounds. To be called after the background element (and with it all other elements) have been repositioned.
    /// This happens, for example, after resizing the window.
    fn update_draw_bounds(&mut self, element_registry: &ElementRegistry) -> UpdateTargetCollection;
}

pub trait LayoutBuilder {
    fn build(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, asset_manager: &mut AssetManager) -> Result<(Box<dyn Layout>, UpdateTargetCollection), String>;
}

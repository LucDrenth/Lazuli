use crate::{graphics::ui::{UiWidgetId, ElementRegistry, interface::WidgetRegistry, UiUpdateTargets, UpdateTargetCollection}, input::Input, ResourceId, asset_manager::AssetManager};

/// Widgets in a layout should get a higher z_index than the background
pub const LAYOUT_ELEMENT_EXTRA_Z_INDEX: f32  = 0.1;

pub trait Layout {
    fn add_widget(&mut self, widget_id: &ResourceId<UiWidgetId>, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry);
    fn update(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, input: &Input, scroll_speed: f32);
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool>;

    /// width is used as fixed: `Width::Fixed(width)`
    fn set_width(&mut self, width: f32, element_registry: &mut ElementRegistry) -> UpdateTargetCollection;
}

pub trait LayoutBuilder {
    fn build(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, asset_manager: &mut AssetManager) -> Result<(Box<dyn Layout>, UpdateTargetCollection), String>;
}

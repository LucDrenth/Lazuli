use glam::Vec2;

use crate::{asset_manager::AssetManager, graphics::ui::{interface::{LayoutRegistry, WidgetRegistry}, ElementRegistry, Position, UiElementId, UiLayoutId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection}, input::Input, ResourceId};

/// Widgets in a layout should get a higher z_index than the background
pub const LAYOUT_ELEMENT_EXTRA_Z_INDEX: f32  = 0.1;
pub const LAYOUT_SCROLLBAR_EXTRA_Z_INDEX: f32 = LAYOUT_ELEMENT_EXTRA_Z_INDEX + 0.5;

pub trait Layout {
    fn add_widget(&mut self, widget_id: &ResourceId<UiWidgetId>, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, layout_registry: &LayoutRegistry) -> UpdateTargetCollection;
    fn add_layout(&mut self, layout_id: &ResourceId<UiLayoutId>, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, layout_registry: &LayoutRegistry) -> UpdateTargetCollection;
    fn update(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, layout_registry: &LayoutRegistry, input: &Input, scroll_speed: f32) -> UpdateTargetCollection;
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32>;
    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool>;
    fn set_position(&mut self, position: Position, element_registry: &mut ElementRegistry) -> UpdateTargetCollection;
    /// The width parameter is used as fixed: `Width::Fixed(width)`
    fn set_width(&mut self, width: f32, element_registry: &mut ElementRegistry) -> UpdateTargetCollection;
    /// Recalculate (and set) max scroll using internal values. Should be called after list of layout elements changed or any layout element has
    /// been resized. Can not be called internally since element resizing is handled through UpdateTargetCollection.
    fn update_max_scroll(&mut self, element_registry: &mut ElementRegistry, widget_registry: &WidgetRegistry, layout_registry: &LayoutRegistry);
    /// Update elements their draw bounds. To be called after the background element (and with it all other elements) have been repositioned.
    /// This happens, for example, after resizing the window.
    fn update_draw_bounds(&mut self, element_registry: &ElementRegistry) -> UpdateTargetCollection;

    fn get_direct_element_ids(&self) -> Vec<ResourceId<UiElementId>>;
    fn get_direct_widget_ids(&self) -> Vec<ResourceId<UiWidgetId>>;
    fn get_direct_layout_ids(&self) -> Vec<ResourceId<UiLayoutId>>;

    /// Get the id of the main element, which can be used for positioning, anchoring etcetera.
    /// It is usually the background element.
    fn get_main_element_id(&self) -> ResourceId<UiElementId>;
    fn get_size(&self, element_registry: &ElementRegistry) -> Option<Vec2>;
    fn get_position(&self) -> Position;
}

pub trait LayoutBuilder {
    fn build(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, layout_registry: &LayoutRegistry, asset_manager: &mut dyn AssetManager) -> Result<(Box<dyn Layout>, UpdateTargetCollection), String>;
}

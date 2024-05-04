use crate::{graphics::ui::{interface::WidgetRegistry, ElementRegistry, Position, UiElementId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection}, input::Input, ResourceId};

use super::Layout;

pub struct MockLayout {
    widgets: Vec<ResourceId<UiWidgetId>>,
    elements: Vec<ResourceId<UiElementId>>,
}

impl Default for MockLayout {
    fn default() -> Self {
        Self { 
            widgets: Default::default() ,
            elements: Default::default(),
        }
    }
}

impl Layout for MockLayout {
    fn add_widget(&mut self, widget_id: &ResourceId<UiWidgetId>, _element_registry: &mut ElementRegistry, _widget_registry: &mut WidgetRegistry) -> UpdateTargetCollection {
        self.widgets.push(widget_id.clone());
        
        Default::default()
    }

    fn update(&mut self, _element_registry: &mut ElementRegistry, _widget_registry: &mut WidgetRegistry, _input: &Input, _scroll_speed: f32) -> UpdateTargetCollection {
        Default::default()
    }

    fn set_z_index(&mut self, _z_index: f32, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        Default::default()
    }

    fn set_visibility(&mut self, _visible: bool, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        Default::default()
    }

    fn set_position(&mut self, _position: Position, _element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        Default::default()
    }

    fn set_width(&mut self, _width: f32, _element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        Default::default()
    }

    fn update_max_scroll(&mut self, _element_registry: &mut ElementRegistry, _widget_registry: &WidgetRegistry) {
    }

    fn update_draw_bounds(&mut self, _element_registry: &ElementRegistry) -> UpdateTargetCollection {
        Default::default()
    }

    fn widgets(&self) -> Vec<ResourceId<UiWidgetId>> {
        self.widgets.clone()
    }
    
    fn get_direct_element_ids(&self) -> Vec<ResourceId<UiElementId>> {
        self.elements.clone()
    }
}

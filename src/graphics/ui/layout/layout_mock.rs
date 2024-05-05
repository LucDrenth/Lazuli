use glam::Vec2;

use crate::{graphics::ui::{interface::{LayoutRegistry, WidgetRegistry}, ElementRegistry, Position, UiElementId, UiLayoutId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection}, input::Input, ResourceId};

use super::{layout_children::LayoutChildren, Layout};

pub struct MockLayout {
    main_element_id: ResourceId<UiElementId>,
    elements: Vec<ResourceId<UiElementId>>,
    children: LayoutChildren,
    size: Vec2,
    position: Position,
}

impl Default for MockLayout {
    fn default() -> Self {
        Self { 
            elements: Default::default(),
            children: Default::default(),
            main_element_id: ResourceId::new(0),
            size: Vec2::ZERO,
            position: Position::Fixed(0., 0.),
        }
    }
}

impl Layout for MockLayout {
    fn add_widget(&mut self, widget_id: &ResourceId<UiWidgetId>, _element_registry: &mut ElementRegistry, _widget_registry: &mut WidgetRegistry, layout_registry: &LayoutRegistry) -> UpdateTargetCollection {
        self.children.push_widget(widget_id.clone());
        
        Default::default()
    }

    fn add_layout(&mut self, layout_id: &ResourceId<crate::graphics::ui::UiLayoutId>, _element_registry: &mut ElementRegistry, _widget_registry: &mut WidgetRegistry, layout_registry: &LayoutRegistry) -> UpdateTargetCollection {
        self.children.push_layout(layout_id.clone());
        
        Default::default()
    }

    fn update(&mut self, _element_registry: &mut ElementRegistry, _widget_registry: &mut WidgetRegistry, _layout_registry: &LayoutRegistry, _input: &Input, _scroll_speed: f32) -> UpdateTargetCollection {
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

    fn update_max_scroll(&mut self, _element_registry: &mut ElementRegistry, _widget_registry: &WidgetRegistry, _layout_registry: &LayoutRegistry) {
    }

    fn update_draw_bounds(&mut self, _element_registry: &ElementRegistry) -> UpdateTargetCollection {
        Default::default()
    }
    
    fn get_direct_element_ids(&self) -> Vec<ResourceId<UiElementId>> {
        self.elements.clone()
    }

    fn get_direct_widget_ids(&self) -> Vec<ResourceId<UiWidgetId>> {
        self.children.widgets()
    }

    fn get_direct_layout_ids(&self) -> Vec<ResourceId<UiLayoutId>> {
        self.children.layouts()
    }
    
    fn get_main_element_id(&self) -> ResourceId<UiElementId> {
        self.main_element_id
    }
    
    fn get_size(&self, element_registry: &ElementRegistry) -> Option<glam::Vec2> {
        Some(self.size)
    }
    
    fn get_position(&self) -> Position {
        self.position
    }

    
}

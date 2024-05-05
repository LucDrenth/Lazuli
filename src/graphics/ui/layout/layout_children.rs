use std::any::TypeId;

use glam::Vec2;

use crate::{graphics::ui::{interface::{LayoutRegistry, WidgetRegistry}, ElementRegistry, Position, UiElementId, UiLayoutId, UiWidgetId}, log, ResourceId};

/// TODO can we make this child enum?
#[derive(Clone)]
pub struct LayoutChildren {
    widget_ids: Vec<ResourceId<UiWidgetId>>,
    layout_ids: Vec<ResourceId<UiLayoutId>>,
    ordered_children: Vec<(TypeId, usize)>, // Type of the element (widget or layout) and its index in the list
}

impl LayoutChildren {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.ordered_children.len()
    }

    pub fn widgets(&self) -> Vec<ResourceId<UiWidgetId>> {
        self.widget_ids.clone()
    }

    pub fn layouts(&self) -> Vec<ResourceId<UiLayoutId>> {
        self.layout_ids.clone()
    }

    pub fn get(&self, index: usize) -> Option<(TypeId, usize)> {
        if index >= self.ordered_children.len() {
            return None;
        }

        Some(self.ordered_children[index])
    }

    pub fn get_id_as_number(&self, index: usize) -> Option<u32> {
        if index >= self.ordered_children.len() {
            return None;
        }

        let (type_id, list_index) = self.ordered_children[index];

        if type_id == TypeId::of::<ResourceId<UiWidgetId>>() {
            Some(*self.widget_ids[list_index].id())
        } else if type_id == TypeId::of::<ResourceId<UiLayoutId>>() {
            Some(*self.layout_ids[list_index].id())
        } else {
           None
        }
    }

    pub fn push_widget(&mut self, widget_id: ResourceId<UiWidgetId>) {
        if self.widget_ids.iter().find(|w| w.equals(&widget_id)).is_some() {
            return;
        }

        self.widget_ids.push(widget_id);

        let index = self.widget_ids.len() - 1;
        let type_id = TypeId::of::<ResourceId<UiWidgetId>>();
        self.ordered_children.push((type_id, index));
    }

    pub fn push_layout(&mut self, layout_id: ResourceId<UiLayoutId>) {
        if self.layout_ids.iter().find(|l| l.equals(&layout_id)).is_some() {
            return;
        }

        self.layout_ids.push(layout_id);

        let index = self.widget_ids.len() - 1;
        let type_id = TypeId::of::<ResourceId<UiLayoutId>>();
        self.ordered_children.push((type_id, index));
    }

    pub fn get_main_element_id(&self, child_index: usize, widget_registry: &WidgetRegistry, layout_registry: &LayoutRegistry) -> Option<ResourceId<UiElementId>> {
        if child_index >= self.len() {
            return None
        }

        let (child_type_id, list_index) = self.ordered_children[child_index];
        
        if child_type_id == TypeId::of::<ResourceId<UiWidgetId>>() {
            widget_registry.get_widget_main_element_id(&self.widget_ids[list_index])
        } else if child_type_id == TypeId::of::<ResourceId<UiLayoutId>>() {
            match layout_registry.get_layout(&self.layout_ids[list_index]) {
                Some(layout) => Some(layout.get_main_element_id()),
                None => None,
            }
        } else {
            log::engine_warn(format!("LayoutChildren.get_main_element_id - unhandled type {:?}", child_type_id));
            None
        }
    }

    pub fn get_element_size(&self, child_index: usize, element_registry: &ElementRegistry, widget_registry: &WidgetRegistry, layout_registry: &LayoutRegistry) -> Option<Vec2> {
        if child_index >= self.len() {
            return None
        }

        let (child_type_id, list_index) = self.ordered_children[child_index];

        if child_type_id == TypeId::of::<ResourceId<UiWidgetId>>() {
            let widget_id = &self.widget_ids[list_index];
            widget_registry.get_widget_size(widget_id, element_registry).ok()
        } else if child_type_id == TypeId::of::<ResourceId<UiLayoutId>>() {
            match layout_registry.get_layout(&self.layout_ids[list_index]) {
                Some(layout) => layout.get_size(element_registry),
                None => None,
            }

        } else {
            log::engine_warn(format!("LayoutChildren.get_element_size - unhandled type {:?}", child_type_id));
            None
        }
    }

    pub fn get_element_position(&self, child_index: usize, element_registry: &ElementRegistry, widget_registry: &WidgetRegistry, layout_registry: &LayoutRegistry) -> Option<Position> {
        if child_index >= self.len() {
            return None
        }

        let (child_type_id, list_index) = self.ordered_children[child_index];

        if child_type_id == TypeId::of::<ResourceId<UiWidgetId>>() {
            let widget_id = &self.widget_ids[list_index];
            widget_registry.get_widget_position(widget_id, element_registry).ok()
        } else if child_type_id == TypeId::of::<ResourceId<UiLayoutId>>() {
            return layout_registry.get_layout(&self.layout_ids[list_index]).map(
                |layout|layout.get_position()
            );
        } else {
            log::engine_warn(format!("LayoutChildren.get_element_position - unhandled type {:?}", child_type_id));
            None
        }
    }

    pub fn get_element_screen_coordinates(&self, child_index: usize, element_registry: &ElementRegistry, widget_registry: &WidgetRegistry, layout_registry: &LayoutRegistry) -> Option<Vec2> {
        if child_index >= self.len() {
            return None
        }

        let (child_type_id, list_index) = self.ordered_children[child_index];

        if child_type_id == TypeId::of::<ResourceId<UiWidgetId>>() {
            let widget_id = &self.widget_ids[list_index];
            widget_registry.get_widget_screen_coordinates(widget_id, element_registry).ok()
        } else if child_type_id == TypeId::of::<ResourceId<UiLayoutId>>() {
            self.get_main_element_id(child_index, widget_registry, layout_registry)
                .and_then(|main_element_id| element_registry.get_element_screen_coordinates(&main_element_id).ok())
        } else {
            log::engine_warn(format!("LayoutChildren.get_element_screen_coordinates - unhandled type {:?}", child_type_id));
            None
        }
    }
}

impl Default for LayoutChildren {
    fn default() -> Self {
        Self { widget_ids: Default::default(), layout_ids: Default::default(), ordered_children: Default::default() }
    }
}

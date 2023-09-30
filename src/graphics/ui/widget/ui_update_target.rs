use crate::{ResourceId, graphics::ui::{UiWidgetId, UiLayoutId}};

/// Because we can not pass the WidgetRegistry to the widgets their mutable functions, we need to have the
/// widgets pass a list of Data elements back to the WidgetRegistry so it can handle all of them within the
/// original function that got called from the widget registry.
/// We can not pass the WidgetRegistry to these functions because the WidgetRegistry looks up the mutable widget
/// and then performs on action on it. Then passing the WidgetRegistry would be borrowing the WidgetRegistry as
/// mutable twice.
pub struct WidgetUpdateTarget<T> {
    pub widget_id: ResourceId<UiWidgetId>,
    pub data: T,
}
impl <T> WidgetUpdateTarget<T> {
    pub fn new(widget_id: ResourceId<UiWidgetId>, data: T) -> Self {
        Self { widget_id, data }
    }
}

pub struct LayoutUpdateTarget<T> {
    pub layout_id: ResourceId<UiLayoutId>,
    pub data: T,
}
impl <T> LayoutUpdateTarget<T> {
    pub fn new(layout_id: ResourceId<UiLayoutId>, data: T) -> Self {
        Self { layout_id, data }
    }
}

pub struct UiUpdateTargets<T> {
    pub widgets: Vec<WidgetUpdateTarget<T>>,
    pub layouts: Vec<LayoutUpdateTarget<T>>,
}
impl <T> Default for UiUpdateTargets<T> {
    fn default() -> Self {
        Self { widgets: Default::default(), layouts: Default::default(), }
    }
}

impl <T> UiUpdateTargets<T> {
    pub fn from_widget_id(widget_id: ResourceId<UiWidgetId>, data: T) -> Self {
        Self {
            widgets: vec![ WidgetUpdateTarget::new(widget_id, data) ],
            layouts: vec![],
        }
    }

    pub fn from_layout_id(layout_id: ResourceId<UiLayoutId>, data: T) -> Self {
        Self {
            widgets: vec![],
            layouts: vec![ LayoutUpdateTarget::new(layout_id, data) ],
        }
    }
}

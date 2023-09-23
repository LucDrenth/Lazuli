use crate::{ResourceId, graphics::ui::UiWidgetId};

/// Because we can not pass the WidgetRegistry to the widgets their mutable functions, we need to have the
/// widgets pass a list of Data elements back to the WidgetRegistry so it can handle all of them within the
/// original function that got called from the widget registry.
/// We can not pass the WidgetRegistry to these functions because the WidgetRegistry looks up the mutable widget
/// and then performs on action on it. Then passing the WidgetRegistry would be borrowing the WidgetRegistry as
/// mutable twice.
/// 
/// TODO - Make 1 generic instead of multiple structs
/// 

pub struct WidgetUpdateTarget<T> {
    pub widget_id: ResourceId<UiWidgetId>,
    pub data: T,
}

impl <T> WidgetUpdateTarget<T> {
    pub fn new(widget_id: ResourceId<UiWidgetId>, data: T) -> Self {
        Self { widget_id, data }
    }
}

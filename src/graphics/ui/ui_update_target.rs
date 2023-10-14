use crate::{ResourceId, graphics::ui::{UiWidgetId, UiLayoutId, Position, bounds_2d::Bounds2d}};

/// Because we can not pass the WidgetRegistry to the widgets their mutable functions, we need to have the
/// widgets pass a list of Data elements back to the WidgetRegistry so it can handle all of them within the
/// original function that got called from the widget registry.
/// We can not pass the WidgetRegistry to these functions because the WidgetRegistry looks up the mutable widget
/// and then performs on action on it. Then passing the WidgetRegistry would be borrowing the WidgetRegistry as
/// mutable twice.
#[derive(Clone)]
pub struct WidgetUpdateTarget<T: Clone> {
    pub widget_id: ResourceId<UiWidgetId>,
    pub data: T,
}
impl <T: Clone> WidgetUpdateTarget<T> {
    pub fn new(widget_id: ResourceId<UiWidgetId>, data: T) -> Self {
        Self { widget_id, data }
    }
}

#[derive(Clone)]
pub struct LayoutUpdateTarget<T> {
    pub layout_id: ResourceId<UiLayoutId>,
    pub data: T,
}
impl <T> LayoutUpdateTarget<T> {
    pub fn new(layout_id: ResourceId<UiLayoutId>, data: T) -> Self {
        Self { layout_id, data }
    }
}

#[derive(Clone)]
pub struct UiUpdateTargets<T: Clone> {
    pub widgets: Vec<WidgetUpdateTarget<T>>,
    pub layouts: Vec<LayoutUpdateTarget<T>>,
}
impl <T: Clone> Default for UiUpdateTargets<T> {
    fn default() -> Self {
        Self { widgets: Default::default(), layouts: Default::default(), }
    }
}

impl <T: Clone> UiUpdateTargets<T> {
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

    pub fn append(&mut self, mut target: UiUpdateTargets<T>) {
        self.widgets.append(&mut target.widgets);
        self.layouts.append(&mut target.layouts);
    }
}


pub struct UpdateTargetCollection {
    pub positions: UiUpdateTargets<Position>,
    pub z_index: UiUpdateTargets<f32>,
    pub draw_bounds: UiUpdateTargets<Bounds2d>,
    pub width: UiUpdateTargets<f32>,
    pub height: UiUpdateTargets<f32>,
    pub visibility: UiUpdateTargets<bool>,
    pub layouts_to_update_draw_bounds: Vec<ResourceId<UiLayoutId>>,
}
impl Default for UpdateTargetCollection {
    fn default() -> Self {
        Self { 
            positions: Default::default(),
            z_index: Default::default(),
            draw_bounds: Default::default(),
            width: Default::default(),
            height: Default::default(),
            visibility: Default::default(),
            layouts_to_update_draw_bounds: vec![],
        }
    }
}

impl UpdateTargetCollection {
    pub fn append(&mut self, mut target: UpdateTargetCollection) {
        self.positions.append(target.positions);
        self.z_index.append(target.z_index);
        self.draw_bounds.append(target.draw_bounds);
        self.width.append(target.width);
        self.height.append(target.height);
        self.visibility.append(target.visibility);
        self.layouts_to_update_draw_bounds.append(&mut target.layouts_to_update_draw_bounds);
    }
}

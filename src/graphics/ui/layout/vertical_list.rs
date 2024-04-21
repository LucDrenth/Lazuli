use glam::Vec2;

use crate::{asset_manager::AssetManager, graphics::{ui::{bounds_2d::Bounds2d, interface::{self, WidgetRegistry}, padding::Padding, shapes::RectangleBuilder, AnchorPoint, ElementRegistry, Position, UiElementId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection, WidgetUpdateTarget}, Color}, input::{Input, MouseButton}, log, ResourceId};

use super::{Layout, layout::{LAYOUT_ELEMENT_EXTRA_Z_INDEX, LAYOUT_SCROLLBAR_EXTRA_Z_INDEX, LayoutBuilder}};

pub struct VerticalList {
    // List of unique ids. We use a Vec instead some Set-like data structure because the order matters.
    widget_ids: Vec<ResourceId<UiWidgetId>>,
    background_element_id: ResourceId<UiElementId>,
    scrollbar_element_id: Option<ResourceId<UiElementId>>,
    scrollbar_inset: Vec2,
    /// The amount of space between elements
    gap_size: f32, 
    position: Position,
    max_height: f32,
    current_scroll: f32,
    max_scroll: f32,
    scroll_on_start_scrollbar_drag: f32,
    padding: Padding,
    draw_bounds: Bounds2d,
    z_index: f32,
    resize_widgets: bool,
    is_visible: bool,
}

impl Layout for VerticalList {
    fn add_widget(&mut self, widget_id: &ResourceId<UiWidgetId>, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry) -> UpdateTargetCollection {
        // Calculate position of newly added widget
        let anchor_id;
        let anchor_point;

        if self.widget_ids.is_empty() {
            anchor_id = self.background_element_id;
            anchor_point = AnchorPoint::TopInside(self.gap_size);
        } else {
            let anchor_widget_id = self.widget_ids.last().unwrap().clone();
            anchor_id = widget_registry.get_widget_main_element_id(&anchor_widget_id).unwrap();
            anchor_point = AnchorPoint::BottomOutside(self.gap_size);
        }

        // Add our new widget to the list if widgets if we don't already have it
        let mut update_targets = UpdateTargetCollection::default();

        for existing_widget_id in self.widget_ids.iter() {
            if existing_widget_id.equals(widget_id) {
                log::engine_warn( format!("VerticalList: widget with id {} was already in the list", widget_id.id()) );
                return update_targets;
            }
        }

        self.widget_ids.push(widget_id.duplicate());

        // Resize background element
        let old_background_size = element_registry.get_element_size(&self.background_element_id).unwrap();
        let new_background_height = calculate_background_height(
            &self.widget_ids, self.gap_size, &self.padding, &element_registry, &widget_registry
        ).min(self.max_height);
        
        if old_background_size.y != new_background_height {
            _ = element_registry.set_rectangle_height(
                &self.background_element_id, 
                new_background_height
            );
        }

        // Set update targets for newly added widget
        update_targets.width.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), old_background_size.x - self.padding.horizontal()));
        update_targets.positions.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), Position::ElementAnchor(anchor_point, anchor_id)));
        update_targets.z_index.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), self.z_index + LAYOUT_ELEMENT_EXTRA_Z_INDEX));
        update_targets.draw_bounds.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), self.draw_bounds.clone()));

        // New max scroll and background height will be calculated after the update_targets have been processed

        update_targets
    }

    fn update_max_scroll(&mut self, element_registry: &mut ElementRegistry, widget_registry: &WidgetRegistry) {
        let background_height = element_registry.get_element_size(&self.background_element_id).unwrap().x;
        self.max_scroll = calculate_max_scroll(&self.widget_ids, &self.padding, &self.background_element_id, background_height, &element_registry, &widget_registry);
        self.update_scroll_bar(element_registry);
    }

    fn update(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, input: &Input, scroll_speed: f32) -> UpdateTargetCollection {
        let mut update_targets = UpdateTargetCollection::default();

        let new_scroll_amount = self.calculate_new_scroll_amount(&element_registry, input, scroll_speed);
        self.update_scroll(new_scroll_amount, &mut update_targets, element_registry, widget_registry, input);

        update_targets
    }

    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.z_index = z_index;

        element_registry.set_element_z_index(&self.background_element_id, z_index).unwrap_or_else(|err| {
            log::engine_warn(format!("failed to set z-index for VerticalList background element with id {:?}: {}", self.background_element_id.id(), err));
        });

        match &self.scrollbar_element_id {
            Some(scrollbar_element_id) => {
                element_registry.set_element_z_index(scrollbar_element_id, z_index + LAYOUT_SCROLLBAR_EXTRA_Z_INDEX).unwrap_or_else(|err| {
                    log::engine_warn(format!("failed to set z-index for VerticalList scrollbar element with id {:?}: {}", scrollbar_element_id.id(), err));
                });
            },
            None => {},
        } 
        
        let mut update_targets: UiUpdateTargets<f32> = Default::default();
        for widget_id in self.widget_ids.iter() {
            update_targets.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), self.z_index + LAYOUT_ELEMENT_EXTRA_Z_INDEX));
        }
        update_targets
    }

    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        _ = element_registry.set_element_visibility(&self.background_element_id, visible).unwrap_or_else(|err| {
            log::engine_warn(format!("failed to set visibility for VerticalList background element with id {:?}: {}", &self.background_element_id.id(), err));
        });

        match &self.scrollbar_element_id {
            Some(scrollbar_element_id) => {
                element_registry.set_element_visibility(scrollbar_element_id, visible).unwrap_or_else(|err| {
                    log::engine_warn(format!("failed to set visibility for VerticalList scrollbar element with id {:?}: {}", scrollbar_element_id.id(), err));
                });
            },
            None => {},
        }

        let mut update_targets: UiUpdateTargets<bool> = Default::default();
        for widget_id in self.widget_ids.iter() {
            update_targets.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), visible));
        }
        update_targets
    }

    fn set_width(&mut self, width: f32, element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        _ = element_registry.set_rectangle_width(&self.background_element_id, width);

        // calculate new draw bounds
        let layout_position = element_registry.get_ui_element_by_id(&self.background_element_id).unwrap().world_data().position();
        let background_size = element_registry.get_element_size(&self.background_element_id).unwrap();
        self.draw_bounds = calculate_draw_bounds(layout_position, background_size);

        let mut update_targets: UpdateTargetCollection = Default::default();
        for widget_id in self.widget_ids.iter() {
            update_targets.width.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), width - self.padding.horizontal()));
            update_targets.draw_bounds.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), self.draw_bounds.clone()));
        }

        update_targets
    }

    fn set_position(&mut self, position: Position, element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        self.position = position;
        _ = element_registry.set_element_position(&self.background_element_id, position);

        self.update_draw_bounds(element_registry)
    }

    fn update_draw_bounds(&mut self, element_registry: &ElementRegistry) -> UpdateTargetCollection {
        let layout_position = element_registry.get_ui_element_by_id(&self.background_element_id).unwrap().world_data().position();
        let background_size = element_registry.get_element_size(&self.background_element_id).unwrap();
        self.draw_bounds = calculate_draw_bounds(layout_position, background_size);

        let mut update_targets: UpdateTargetCollection = Default::default();
        for widget_id in self.widget_ids.iter() {
            update_targets.draw_bounds.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), self.draw_bounds.clone()));
        }

        update_targets
    }

    fn widgets(&self) -> Vec<ResourceId<UiWidgetId>> {
        self.widget_ids.clone()
    }
}

impl VerticalList {
    fn calculate_new_scroll_amount(&mut self, element_registry: &ElementRegistry, input: &Input, scroll_speed: f32) -> f32 {
        let scrollbar_element_id = {
            match self.scrollbar_element_id {
                Some(scrollbar_element_id) => scrollbar_element_id,
                None => return self.calculate_new_scroll_amount_from_scroll(input, scroll_speed),
            }
        };

        match element_registry.get_ui_element_by_id(&scrollbar_element_id) {
            Some(element) => {
                if element.world_data().event_handlers.mouse_left_drag_handler.is_handling() {
                    if element.world_data().event_handlers.mouse_left_drag_handler.did_drag_start() {
                        self.scroll_on_start_scrollbar_drag = self.current_scroll;
                    }

                    let amount_moved = input.get_mouse_position_y() as f32 - element.world_data().event_handlers.mouse_left_drag_handler.get_drag_start_position().y;
                    let extra_scroll = (amount_moved / self.max_height) * (self.max_scroll + self.max_height);
                    return (self.scroll_on_start_scrollbar_drag + extra_scroll).clamp(0.0, self.max_scroll);
                }
            },
            None => {
                log::engine_warn(format!("VerticalList failed to check scroll from drag because scrollbar with id {} was not found", scrollbar_element_id.id()));
            },
        }

        return self.calculate_new_scroll_amount_from_scroll(input, scroll_speed);
    }

    fn calculate_new_scroll_amount_from_scroll(&self, input: &Input, scroll_speed: f32) -> f32 {
        (self.current_scroll - input.get_scroll_y() as f32 * scroll_speed).clamp(0.0, self.max_scroll)
    }

    fn update_scroll(&mut self, new_scroll_amount: f32, update_target_collection: &mut UpdateTargetCollection, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, input: &Input) {
        if !self.should_update_scroll(element_registry, input) {
            return;
        }

        if self.current_scroll == new_scroll_amount {
            return;
        }
        self.current_scroll = new_scroll_amount;

        update_target_collection.append(self.update_scroll_for_widgets(element_registry, widget_registry));
        self.update_scroll_bar(element_registry);
    }

    fn should_update_scroll(&self, element_registry: &ElementRegistry, input: &Input) -> bool {
        if self.widget_ids.is_empty() && input.get_scroll_y() == 0.0 {
            return false;
        }

        match element_registry.get_ui_element_by_id(&self.background_element_id) {
            Some(element) => {
                if element.world_data().event_handlers.scroll_handler.did_handle() {
                    return true;
                }
            },
            None => {
                log::engine_warn(format!("VerticalLst does not update scroll because background element (id={}) was not found", self.background_element_id.id()));
            },
        }

        match self.scrollbar_element_id {
            Some(scrollbar_element_id) => {
                return element_registry.is_element_dragged(&scrollbar_element_id, MouseButton::Left);
            },
            None => return false,
        }
    }

    fn update_scroll_for_widgets(&self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry) -> UpdateTargetCollection {
        let mut update_targets = UpdateTargetCollection::default();

        // Update widgets position
        let first_widget_id = self.widget_ids.first().unwrap();
        let first_widget_anchor_element_id  = widget_registry.get_widget_main_element_id(first_widget_id).unwrap();

        _ = element_registry.set_element_position_transform(
            &first_widget_anchor_element_id, 
            Vec2::new(0.0, self.current_scroll)
        ).map_err(|err|{
            log::engine_err(format!("failed to scroll VerticalList layout because the first widget element [id={}] was not found", err));
        });

        for widget_id in &self.widget_ids {
            update_targets.update_draw_bounds_recursively.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), ()))
        }
        
        update_targets
    }

    fn update_scroll_bar(&self, element_registry: &mut ElementRegistry) {
        match self.scrollbar_element_id {
            Some(scrollbar_element_id) => {
                let progress = self.current_scroll / self.max_scroll;
                let max_scrollbar_height = self.max_height - self.scrollbar_inset.y * 2.0;
                let mut new_scrollbar_height = self.max_height / (self.max_scroll + self.max_height) * max_scrollbar_height;
                new_scrollbar_height = new_scrollbar_height.max(interface::minimum_scrollbar_size());

                match element_registry.get_element_size(&scrollbar_element_id) {
                    Ok(current_scrollbar_size) => {
                        if current_scrollbar_size.y != new_scrollbar_height {
                            _ = element_registry.set_rectangle_height(&scrollbar_element_id, new_scrollbar_height).map_err(|err| {
                                log::engine_err(format!("failed to update scrollbar height because scrollbar element with id {} was not found: {}", scrollbar_element_id.id(), err));
                            });
                        }

                        let max_scrollbar_distance = self.max_height - new_scrollbar_height - self.scrollbar_inset.y * 2.0;
                        let offset = max_scrollbar_distance * progress;
                        _ = element_registry.set_element_position(&scrollbar_element_id, Position::ElementAnchor(
                            AnchorPoint::TopRightInside(self.scrollbar_inset.x, self.scrollbar_inset.y + offset), 
                            self.background_element_id
                        )).map_err(|err| {
                            log::engine_err(format!("failed to scroll scrollbar because background element with id {} was not found: {}", self.background_element_id.id(), err));
                        });
                    },
                    Err(err) => {
                        log::engine_err(format!("failed to scroll scrollbar because scrollbar element with id {} was not found: {}", scrollbar_element_id.id(), err));
                    },
                }
            },
            None => {},
        }
    }
}

pub enum Width {
    /// Fixed width
    Fixed(f32),
    /// Size of the bigest element + horizontal padding, but never higher than the given amount.
    MaxWidth(f32),
    /// Size of the bigest element + horizontal padding
    Auto(),
}

pub struct VerticalListBuilder {
    widget_ids: Vec<ResourceId<UiWidgetId>>,
    gap_size: f32, // the amount of space between elements
    background_color: Color,
    max_height: f32,
    width: Width,
    position: Position,
    padding: Padding,
    z_index: f32,
    resize_widgets: bool,
    is_visible: bool,
    has_scrollbar: bool,
    scrollbar_color: Color,
    scrollbar_width: f32,
    scrollbar_inset: Vec2,
}

impl LayoutBuilder for VerticalListBuilder {
    fn build(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, asset_manager: &mut AssetManager) -> Result<(Box<dyn Layout>, UpdateTargetCollection), String> {
        let (layout, update_targets) = self.build(element_registry, widget_registry, asset_manager)?;
        Ok((Box::new(layout), update_targets))
    }
}

impl VerticalListBuilder {
    pub fn new() -> Self {
        let default_gap_size = 10.0;

        Self {
            widget_ids: vec![],
            gap_size: default_gap_size,
            background_color: Color::black(),
            max_height: 300.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            padding: Padding::Universal(default_gap_size),
            z_index: 100.0,
            resize_widgets: true,
            width: Width::Auto(),
            is_visible: true,
            has_scrollbar: true,
            scrollbar_width: 5.0,
            scrollbar_color: Color::Rgba(255, 255, 255, 0.7),
            scrollbar_inset: Vec2::new(2.0, 2.0),
        }
    }

    pub fn build(&mut self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, asset_manager: &mut AssetManager) -> Result<(VerticalList, UpdateTargetCollection), String> {
        let background_width = match self.width {
            Width::Fixed(width) => width,
            Width::MaxWidth(max_width) => { 
                calculate_background_width(&self.widget_ids, &self.padding, &element_registry, &widget_registry).min(max_width) 
            },
            Width::Auto() => { calculate_background_width(&self.widget_ids, &self.padding, &element_registry, &widget_registry) },
        }; 
        let background_height = calculate_background_height(&self.widget_ids, self.gap_size, &self.padding, &element_registry, &widget_registry).min(self.max_height);

        let background_element_id = element_registry.create_rectangle(&RectangleBuilder::new()
            .with_color(self.background_color.clone())
            .with_position(self.position)
            .with_width(background_width)
            .with_height(background_height)
            .with_z_index(self.z_index)
            .with_visibility(self.is_visible)
            .with_handle_scroll(true)
        , asset_manager)?;

        let scrollbar_element_id = match self.has_scrollbar {
            true => {
                Some(element_registry.create_rectangle(&RectangleBuilder::new()
                    .with_position(Position::ElementAnchor(
                        AnchorPoint::TopRightInside(self.scrollbar_inset.x, self.scrollbar_inset.y), 
                        background_element_id
                    ))
                    .with_width(self.scrollbar_width)
                    .with_height(0.0) // Will be set after the widget positions have been set because only after that we can calculate the maximum scroll
                    .with_color(self.scrollbar_color.clone())
                    .with_z_index(self.z_index + LAYOUT_SCROLLBAR_EXTRA_Z_INDEX)
                    .with_border_radius(self.scrollbar_width / 2.0)
                , asset_manager)?)
            },
            false => None,
        };

        let layout_position = element_registry.get_ui_element_by_id(&background_element_id).unwrap().world_data().position();

        // Take out all of the widget ids
        let mut widget_ids: Vec<ResourceId<UiWidgetId>> = Vec::new();
        widget_ids.extend(self.widget_ids.drain(..));

        let list = VerticalList { 
            widget_ids,
            background_element_id, 
            scrollbar_element_id: scrollbar_element_id,
            scrollbar_inset: self.scrollbar_inset,
            gap_size: self.gap_size, 
            position: self.position, 
            max_height: self.max_height,
            current_scroll: 0.0,
            max_scroll: 0.0, // Will be set after the widget position have been set because only after that we can calculate the maximum scroll
            scroll_on_start_scrollbar_drag: 0.0,
            padding: self.padding.clone(),
            draw_bounds: calculate_draw_bounds(layout_position, Vec2::new(background_width, background_height)),
            z_index: self.z_index,
            resize_widgets: self.resize_widgets,
            is_visible: self.is_visible,
        };

        let mut update_targets = UpdateTargetCollection::default();

        if list.widget_ids.is_empty() {
            return Ok((list, update_targets));
        }

        update_targets.positions.widgets.push(WidgetUpdateTarget::new(
            list.widget_ids[0], 
            Position::ElementAnchor(AnchorPoint::TopInside(list.padding.top()), list.background_element_id)
        ));

        for i in 1..list.widget_ids.len() {
            let anchor_element = widget_registry.get_widget_main_element_id(&list.widget_ids[i - 1]).unwrap();
            
            update_targets.positions.widgets.push(WidgetUpdateTarget::new(
                list.widget_ids[i], 
                Position::ElementAnchor(AnchorPoint::BottomOutside(self.gap_size), anchor_element), 
            ));
        }

        for widget_id in list.widget_ids.iter() {
            update_targets.z_index.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), list.z_index + LAYOUT_ELEMENT_EXTRA_Z_INDEX));
            update_targets.draw_bounds.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), list.draw_bounds.clone()));
            update_targets.width.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), background_width - list.padding.horizontal()));
            update_targets.visibility.widgets.push(WidgetUpdateTarget::new(widget_id.clone(), self.is_visible));
        }

        // list.max_scroll still needs to be calculated, but only after the update targets are handled.

        Ok((list, update_targets))
    }


    /*******************************
     * Setter functions start here *
     *******************************/

    pub fn add_widget(mut self, widget_id: &ResourceId<UiWidgetId>) -> Self {
        self.try_add_widget(widget_id.duplicate());
        self
    }

    pub fn add_widgets(mut self, widget_ids: &Vec<ResourceId<UiWidgetId>>) -> Self {
        // We do not add them all at once because we only want unique values in the list
        for widget_id in widget_ids.iter() {
            self.try_add_widget(widget_id.duplicate());
        }

        self
    }

    fn try_add_widget(&mut self, widget_id: ResourceId<UiWidgetId>) {
        for existing_widget_id in self.widget_ids.iter() {
            if existing_widget_id.equals(&widget_id) {
                log::engine_warn(format!("VerticalListBuilder: widget with id {} was already in the list", widget_id.id()));
                return;
            }
        }

        self.widget_ids.push(widget_id);
    }

    pub fn with_gap_size(mut self, gap_size: f32) -> Self {
        self.gap_size = gap_size;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn with_resize_widgets(mut self, resize_widgets: bool) -> Self {
        self.resize_widgets = resize_widgets;
        self
    }

    pub fn with_width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.is_visible = visible;
        self
    }

    pub fn with_scrollbar(mut self, has_scrollbar: bool) -> Self {
        self.has_scrollbar = has_scrollbar;
        self
    }

    pub fn with_scrollbar_width(mut self, scrollbar_width: f32) -> Self {
        self.scrollbar_width = scrollbar_width;
        self
    }

    pub fn with_scrollbar_color(mut self, color: Color) -> Self {
        self.scrollbar_color = color;
        self
    }

    pub fn with_scrollbar_inset(mut self, scrollbar_inset: Vec2) -> Self {
        self.scrollbar_inset = scrollbar_inset;
        self
    }
    pub fn with_scrollbar_inset_x(mut self, scrollbar_inset_x: f32) -> Self {
        self.scrollbar_inset.x = scrollbar_inset_x;
        self
    }
    pub fn with_scrollbar_inset_y(mut self, scrollbar_inset_y: f32) -> Self {
        self.scrollbar_inset.y = scrollbar_inset_y;
        self
    }
}

fn calculate_background_height(widget_ids: &Vec<ResourceId<UiWidgetId>>, gap_size: f32, padding: &Padding, element_registry: &ElementRegistry, widget_registry: &WidgetRegistry) -> f32 {
    let mut background_height = padding.top();

    for widget_id in widget_ids.iter() {
        background_height += widget_registry.get_widget_size(widget_id, element_registry).unwrap().y + gap_size;
    }

    background_height - gap_size + padding.bottom()
}

/// Returns the width of widest widget + two times the gap size as padding, or only the gap size if there are no elements
fn calculate_background_width(widget_ids: &Vec<ResourceId<UiWidgetId>>, padding: &Padding, element_registry: &ElementRegistry, widget_registry: &WidgetRegistry) -> f32 {
    if widget_ids.is_empty() {
        return padding.horizontal();
    }

    let mut widest: f32 = 0.0;

    for widget_id in widget_ids.iter() {
        widest = widest.max(
            widget_registry.get_widget_size(widget_id, element_registry).unwrap().x
        )
    }

    widest + padding.horizontal()
}

fn calculate_max_scroll(
    widget_ids: &Vec<ResourceId<UiWidgetId>>, 
    padding: &Padding, 
    background_element_id: &ResourceId<UiElementId>, 
    layout_height: f32, 
    element_registry: &ElementRegistry, 
    widget_registry: &WidgetRegistry
) -> f32 {
    if widget_ids.is_empty() {
        return 0.0;
    }

    let first_widget_id = widget_ids.first().unwrap();
    let last_widget_id = widget_ids.last().unwrap();

    let last_element_size = widget_registry.get_widget_size(last_widget_id, element_registry).unwrap();
    let last_element_position = widget_registry.get_widget_screen_position(last_widget_id, element_registry).unwrap();
    let bottom_of_last_element = last_element_position.y + last_element_size.y / 2.0;
            
    let layout_position_y = element_registry.get_element_screen_position(background_element_id).unwrap().y;
    let bottom_of_layout = layout_position_y - layout_height / 2.0;

    let max_scroll = (bottom_of_last_element - bottom_of_layout).abs() 
        + padding.bottom() 
        + widget_registry.get_widget_position_transform(first_widget_id, element_registry).unwrap().y;
    (max_scroll).max(0.0)
}

fn calculate_draw_bounds(layout_position: Vec2, layout_size: Vec2) -> Bounds2d {
    Bounds2d::some(
        layout_position.y + layout_size.y / 2.0, 
        layout_position.x + layout_size.x / 2.0, 
        layout_position.y - layout_size.y / 2.0, 
        layout_position.x - layout_size.x / 2.0
    )
}

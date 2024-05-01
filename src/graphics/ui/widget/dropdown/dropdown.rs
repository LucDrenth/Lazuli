use std::{fmt::Debug, f32::consts::PI};

use glam::Vec2;
use interface::WidgetRegistryUdpateResult;

use crate::{asset_manager::AssetManager, graphics::{ui::{bounds_2d::Bounds2d, element::InputEvent, interface::{self, is_valid_z_index, LayoutRegistry, WidgetRegistry, MAX_Z_INDEX}, widget::{ButtonBuilder, IconBuilder, UiWidget}, AnchorPoint, ElementRegistry, LayoutUpdateTarget, Position, TextAlign, UiElementId, UiLayoutId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection, VerticalListBuilder, WidgetUpdateTarget, Width}, Color}, input::ButtonAction, log, ResourceId};

struct DropdownOptionButton<T: Debug + Clone> {
    button_id: ResourceId<UiWidgetId>,
    value: T,
    label: String,
}

pub struct Dropdown<T: Debug + Clone> {
    z_index: f32,
    button_id: ResourceId<UiWidgetId>,
    icon_widget_id: ResourceId<UiWidgetId>,
    options: Vec<DropdownOptionButton<T>>,
    options_layout: ResourceId<UiLayoutId>,
    is_open: bool,
    selected: Option<T>,
    /// If false, option buttons do not use draw bounds
    option_buttons_respect_draw_bounds: bool,
    debug: bool,
}

impl <T: Debug + Clone> UiWidget for Dropdown<T> {
    fn get_direct_element_ids(&self) -> Vec<ResourceId<UiElementId>> {
        vec![]
    }
    fn get_direct_layout_ids(&self) -> Vec<ResourceId<UiLayoutId>> {
        vec![ 
            self.options_layout,
        ]
    }
    fn get_direct_widget_ids(&self) -> Vec<ResourceId<UiWidgetId>> {
        vec![
            self.icon_widget_id,
            self.button_id,
        ]
    }

    fn get_main_element_id(&self, widget_registry: &WidgetRegistry) -> ResourceId<UiElementId> {
        widget_registry.get_widget_by_id(&self.button_id).unwrap().get_main_element_id(widget_registry)
    }

    fn z_index(&self) -> f32 {
        self.z_index
    }

    fn set_z_index(&mut self, z_index: f32, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.z_index = z_index;

        UiUpdateTargets {
            widgets: vec![
                WidgetUpdateTarget::new(self.button_id, z_index),
                WidgetUpdateTarget::new(self.icon_widget_id, z_index + 0.01),
            ],
            layouts: vec![
                LayoutUpdateTarget::new(self.options_layout, Self::option_button_z_index(z_index)),
            ],
        }
    }

    fn set_position(&self, position: Position, _element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        let mut targets = UpdateTargetCollection::default();

        // The options layout is anchored to the button, which makes it reposition the elements already. But we also need to
        // update the layout its drawbounds after repositioning its elements.
        targets.positions.widgets.push(WidgetUpdateTarget::new(self.button_id, position));
        targets.update_draw_bounds_recursively.layouts.push(LayoutUpdateTarget::new(self.options_layout, ()));

        targets
    }

    fn set_draw_bounds(&self, draw_bounds: Bounds2d, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d> {
        let mut targets = UiUpdateTargets{
            widgets: vec![
                WidgetUpdateTarget::new(self.button_id, draw_bounds),
                WidgetUpdateTarget::new(self.icon_widget_id, draw_bounds),
            ],
            layouts: vec![],
        };

        if self.option_buttons_respect_draw_bounds {
            targets.layouts.push(LayoutUpdateTarget::new(self.options_layout, draw_bounds));
        }

        targets
    }

    fn set_width(&self, width: f32, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        UiUpdateTargets{
            widgets: vec![
                WidgetUpdateTarget::new(self.button_id, width)
            ],
            layouts: vec![
                LayoutUpdateTarget::new(self.options_layout, width),
            ],
        }
    }
    fn set_height(&self, height: f32, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        UiUpdateTargets{
            widgets: vec![
                WidgetUpdateTarget::new(self.button_id, height)
            ],
            layouts: vec![],
        }
    }
    fn set_size(&self, size: Vec2, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2> {
        UiUpdateTargets{
            widgets: vec![
                WidgetUpdateTarget::new(self.button_id, size)
            ],
            layouts: vec![
                LayoutUpdateTarget::new(self.options_layout, size),
            ],
        }
    }

    fn set_visibility(&mut self, visible: bool, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        UiUpdateTargets { 
            widgets: vec![
                WidgetUpdateTarget::new(self.button_id, visible),
                WidgetUpdateTarget::new(self.icon_widget_id, visible),
            ], 
            layouts: vec![
                LayoutUpdateTarget::new(self.options_layout, false), // only show upon clicking the button
            ],
        }
    }

    fn is_debug(&self) -> bool {
        self.debug
    }
}

impl<T: Debug + Clone> Dropdown<T> {
    fn option_button_z_index(base_z_index: f32) -> f32 {
        (1_000.0 + base_z_index).min(MAX_Z_INDEX - 1.0)
    }

    /// Returns the newly selected value, or None if nothing has changed
    pub fn update(
        &mut self, 
        clicked_button_id: &Option<ResourceId<UiWidgetId>>,
        widget_registry_update_result: &mut WidgetRegistryUdpateResult,
    ) -> Option<T> {
        if let Some(clicked_button) = clicked_button_id {
            if self.is_open {
                for option in self.options.iter() {
                    if clicked_button.equals(&option.button_id) {
                        self.is_open = false;
                        self.handle_state(widget_registry_update_result);

                        widget_registry_update_result.buttons_to_change_text.push(
                            WidgetUpdateTarget::new(self.button_id, option.label.clone())
                        );
                        return Some(option.value.clone());
                    }
                }
            }

            if clicked_button.equals(&self.button_id) {
                self.is_open = !self.is_open;
                self.handle_state(widget_registry_update_result);
            }
        }

        None
    }

    fn handle_state(&self, widget_registry_update_result: &mut WidgetRegistryUdpateResult) {
        let rotation: f32;
        if self.is_open {
            rotation = PI;
        } else {
            rotation = 0.;
        }

        widget_registry_update_result.widgets_to_set_main_element_custom_shader_value_f32.push(WidgetUpdateTarget::new(
            self.icon_widget_id, 
            ("rotation".to_string(), rotation) ,
        ));

        widget_registry_update_result.update_targets_visibility.layouts.push(LayoutUpdateTarget::new(self.options_layout, self.is_open));
    }

    pub fn is_open(&self) -> bool { self.is_open }
}

pub struct DropdownOption<T: Debug + Clone> {
    pub label: String,
    pub value: T,
}

impl <T: Debug + Clone> DropdownOption<T> {
    pub fn new(label: impl Into<String>, value: T) -> Self {
        DropdownOption { label: label.into(), value }
    }
}

pub struct DropdownBuilder<T: Debug + Clone> {
    placeholder_text: Option<String>,
    options: Vec<DropdownOption<T>>,
    initially_selected_index: Option<u32>, // index of the options list
    z_index: f32,
    position: Position,
    option_buttons_respect_draw_bounds: bool,
    text_align: TextAlign,
    text_color: Color,
    gap_size: f32, // space between options
    width: f32,
    debug: bool,
}

impl<T: Debug + Clone> DropdownBuilder<T> {
    pub fn new() -> Self {
        Self {
            placeholder_text: None,
            options: vec![],
            initially_selected_index: Some(0),
            z_index: 10.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            option_buttons_respect_draw_bounds: false,
            text_align: TextAlign::Left,
            text_color: interface::default_text_color(),
            gap_size: 5.0,
            width: 100.0,
            debug: false,
        }
    }

    pub fn build(&self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, layout_registry: &mut LayoutRegistry, asset_manager: &mut dyn AssetManager) -> Result<(Dropdown<T>, Vec<UpdateTargetCollection>), String> {
        self.validate()?;

        let selected_value: Option<T>;
        let label;

        if self.placeholder_text.is_some() {
            label = self.placeholder_text.clone().unwrap();
            selected_value = None;
        } else if self.initially_selected_index.is_some() {
            let selected_option = &self.options[self.initially_selected_index.unwrap() as usize];
            label = selected_option.label.clone();
            selected_value = Some(selected_option.value.clone());
        } else {
            label = "".to_string();
            selected_value = None;
        };

        let button = ButtonBuilder::new()
            .with_position(self.position)
            .with_z_index(self.z_index)
            .with_text_align(self.text_align)
            .with_text_color(self.text_color.clone())
            .with_width(self.width)
            .build(label, element_registry, asset_manager)
        ?;

        let mut options = vec![];
        let button_anchor = button.get_main_element_id(&widget_registry);

        let option_button_builder = ButtonBuilder::new()
            .with_height(button.height())
            .with_mouse_action_to_activate(ButtonAction::UpOrDown)
            .with_text_align(self.text_align)
            .with_text_color(self.text_color.clone())
        ;

        let mut layout_builder = VerticalListBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::BottomOutside(0.), button_anchor))
            .with_background_color(Color::orange())
            .with_visibility(false)
            .with_z_index(Dropdown::<T>::option_button_z_index(self.z_index))
            .with_width(Width::Fixed(self.width))
            .with_scrollbar(false)
        ;

        for option in &self.options {
            let option_button_id = widget_registry.create_button(option.label.clone(), &option_button_builder, element_registry, asset_manager)?;
            layout_builder = layout_builder.add_widget(&option_button_id);
            options.push(DropdownOptionButton{ button_id: option_button_id, value: option.value.clone(), label: option.label.clone() });
        }

        let icon_right = button.padding().right();
        let icon_widget_id = widget_registry.create_icon(&IconBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightInside(icon_right), button_anchor))
            .with_color(self.text_color.clone())
            .with_z_index(self.z_index + 0.01)
            .with_height(6.0)
            .with_handle_input_event(InputEvent::MouseLeftDown, false)
            .with_handle_input_event(InputEvent::MouseLeftUp, false)
        , element_registry, asset_manager)?;

        let (layout, layout_update_targets) = layout_registry.create_layout(&mut layout_builder, element_registry, widget_registry, asset_manager)?;
        let button_id = widget_registry.add_button(button);

        Ok((Dropdown {
            z_index: self.z_index,
            button_id,
            icon_widget_id,
            options,
            options_layout: layout,
            is_open: false,
            selected: selected_value,
            option_buttons_respect_draw_bounds: self.option_buttons_respect_draw_bounds,
            debug: self.debug,
        }, vec![layout_update_targets]))
    }

    pub fn with_placeholder_text(mut self, placeholder_text: impl Into<String>) -> Self {
        self.placeholder_text = Some(placeholder_text.into());
        self.initially_selected_index = None;
        self
    }

    pub fn with_initially_selected_index(mut self, initially_selected_index: u32) -> Self {
        self.initially_selected_index = Some(initially_selected_index);
        self.placeholder_text = None;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.initially_selected_index.is_some() && self.options.len() <= self.initially_selected_index.unwrap() as usize {
            return Err(format!(
                "Initially selected index ({}) is higher than the number of options ({})"
                , self.initially_selected_index.unwrap(), self.options.len()
            ));
        }

        Ok(())
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        // Add 1 because the given z_index is for the lowest element, and higher elements may go up to 1 higher
        if is_valid_z_index(z_index + 1.0) {
            self.z_index = z_index;
        } else {
            log::engine_warn(format!("Did not set ButtonBuilder z_index {} because it's not a valid z-index", z_index));
        }

        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_options(mut self, options: Vec<DropdownOption<T>>) -> Self {
        self.options = options;
        self
    }

    pub fn with_option_buttons_respect_draw_bounds(mut self, option_buttons_respect_draw_bounds: bool) -> Self {
        self.option_buttons_respect_draw_bounds = option_buttons_respect_draw_bounds;
        self
    }

    pub fn with_text_align(mut self, text_align: TextAlign) -> Self {
        self.text_align = text_align;
        self
    }

    pub fn with_gap_size(mut self, gap_size: f32) -> Self {
        self.gap_size = gap_size;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

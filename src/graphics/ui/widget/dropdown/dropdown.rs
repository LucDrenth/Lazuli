use std::{fmt::Debug, f32::consts::PI};

use glam::Vec2;
use interface::WidgetRegistryUdpateResult;

use crate::{asset_manager::AssetManager, graphics::{ui::{ElementRegistry, widget::{Button, ButtonBuilder, UiWidget, IconBuilder, Icon, widget_update_target::WidgetUpdateTarget}, interface::{is_valid_z_index, MAX_Z_INDEX, self, WidgetRegistry}, Position, AnchorPoint, UiElementId, TextAlign, UiWidgetId, bounds_2d::Bounds2d}, Color}, log, input::{Input, InputAction}, ResourceId};

struct DropdownOptionButton<T: Debug + Clone> {
    button_id: ResourceId<UiWidgetId>,
    value: T,
    label: String,
}

pub struct Dropdown<T: Debug + Clone> {
    z_index: f32,
    button: Button, // TODO move to widget registry
    icon_widget: Icon, // TODO move to widget registry
    options: Vec<DropdownOptionButton<T>>,
    is_open: bool,
    selected: Option<T>,
    /// If false, option buttons do not use draw bounds
    option_buttons_respect_draw_bounds: bool,
}

impl <T: Debug + Clone> UiWidget for Dropdown<T> {
    fn get_all_element_ids(&self, widget_registry: &WidgetRegistry) -> Vec<ResourceId<UiElementId>> {
        let mut element_ids = vec![];

        for option in &self.options {
            element_ids.append(&mut widget_registry.get_widget_by_id(&option.button_id).unwrap().get_all_element_ids(widget_registry))
        }

        element_ids.append(&mut self.button.get_all_element_ids(widget_registry));
        element_ids.append(&mut self.icon_widget.get_all_element_ids(widget_registry));

        return element_ids
    }

    fn get_main_element_id(&self) -> ResourceId<UiElementId> {
        self.button.get_main_element_id()
    }

    fn z_index(&self) -> f32 {
        self.z_index
    }

    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry) {
        self.button.set_position(position, element_registry);
    }

    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> Vec<WidgetUpdateTarget<f32>> {
        self.z_index = z_index;
        self.button.set_z_index(z_index, element_registry);
        self.icon_widget.set_z_index(z_index + 0.01, element_registry);

        let mut targets = vec![];
        for option in self.options.iter_mut() {
            targets.push(WidgetUpdateTarget{ 
                widget_id: option.button_id, 
                data: Self::option_button_z_index(z_index) 
            });
        }

        targets
    }

    fn set_draw_bounds(&self, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry) -> Vec<WidgetUpdateTarget<Bounds2d>> {
        _ = self.button.set_draw_bounds(draw_bounds, element_registry);
        _ = self.icon_widget.set_draw_bounds(draw_bounds, element_registry);

        let mut targets = vec![];

        if self.option_buttons_respect_draw_bounds {
            for option in self.options.iter() {
                targets.push(WidgetUpdateTarget { 
                    widget_id: option.button_id,
                    data: draw_bounds,
                })
            }
        }

        targets
    }

    fn set_width(&self, width: f32, element_registry: &mut ElementRegistry) -> Vec<WidgetUpdateTarget<f32>> {
        self.button.set_width(width, element_registry);

        let mut targets = vec![];
        
        for option in &self.options {
            targets.push(WidgetUpdateTarget { 
                widget_id: option.button_id, 
                data: width,
            })
        }

        targets
    }
    fn set_height(&self, height: f32, element_registry: &mut ElementRegistry) -> Vec<WidgetUpdateTarget<f32>> {
        self.button.set_height(height, element_registry);

        let mut targets = vec![];

        for option in &self.options {
            targets.push(WidgetUpdateTarget { 
                widget_id: option.button_id, 
                data: height,
            })
        }

        targets
    }
    fn set_size(&self, size: Vec2, element_registry: &mut ElementRegistry) -> Vec<WidgetUpdateTarget<Vec2>> {
        self.button.set_size(size, element_registry);

        let mut targets = vec![];

        for option in &self.options {
            targets.push(WidgetUpdateTarget { 
                widget_id: option.button_id, 
                data: size,
            })
        }

        targets
    }

    fn on_show(&mut self) {}
    fn on_hide(&mut self) {}
}

impl<T: Debug + Clone> Dropdown<T> {
    fn option_button_z_index(base_z_index: f32) -> f32 {
        (1000.0 + base_z_index).min(MAX_Z_INDEX - 1.0)
    }

    /// Returns the newly selected value, or None if nothing has changed
    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager, clicked_button_id: &Option<ResourceId<UiWidgetId>>, widget_registry_update_result: &mut WidgetRegistryUdpateResult) -> Option<T> {
        if self.is_open {
            if let Some(clicked_button) = clicked_button_id {
                for option in self.options.iter() {
                    if clicked_button.equals(&option.button_id) {
                        let value = option.value.clone();

                        match element_registry.set_text(&self.button.text_element_id(), &option.label, asset_manager) {
                            Ok(_) => (),
                            Err(err) => {
                                log::engine_err(format!("failed to set selected dropdown value {:?}: {}", value, err));
                                return None;
                            },
                        }
                        
                        self.set_open(false, element_registry);
                        widget_registry_update_result.widgets_to_hide.append(&mut self.get_option_button_ids());
                        return Some(value);
                    }
                }
            }
        }

        if self.button.is_clicked(input, element_registry) {
            self.set_open(!self.is_open, element_registry);

            if self.is_open {
                widget_registry_update_result.widgets_to_show.append(&mut self.get_option_button_ids());
            } else {
                widget_registry_update_result.widgets_to_hide.append(&mut self.get_option_button_ids());
            }
        }

        None
    }

    fn get_option_button_ids(&self) -> Vec<ResourceId<UiWidgetId>> {
        let mut ids = vec![];
        for button in self.options.iter() {
            ids.push(button.button_id);
        }
        ids
    }

    fn set_open(&mut self, open: bool, element_registry: &mut ElementRegistry) {
        self.is_open = open;
        if self.is_open {
            element_registry.get_mut_element_custom_shader_values(&self.icon_widget.get_main_element_id()).unwrap().set_f32("rotation", PI);
        } else {
            element_registry.get_mut_element_custom_shader_values(&self.icon_widget.get_main_element_id()).unwrap().set_f32("rotation", 0.)
        }
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
        }
    }

    pub fn build(&self, element_registry: &mut ElementRegistry, widget_registry: &mut WidgetRegistry, asset_manager: &mut AssetManager) -> Result<Dropdown<T>, String> {
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
            .build(label, element_registry, asset_manager)
        ?;

        // TODO add background for options

        let mut options = vec![];
        let mut anchor_element_id = button.get_main_element_id();

        let mut option_button_builder = ButtonBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::BottomOutside(self.gap_size), anchor_element_id))
            .with_width(button.width())
            .with_height(button.height())
            .with_mouse_action_to_activate(InputAction::UpOrDown)
            .with_hidden(true)
            .with_z_index(Dropdown::<T>::option_button_z_index(self.z_index))
            .with_text_align(self.text_align)
            .with_text_color(self.text_color.clone())
        ;

        for option in &self.options {
            option_button_builder = option_button_builder.with_position(Position::ElementAnchor(AnchorPoint::BottomOutside(5.0), anchor_element_id));
            let option_button = option_button_builder.build(option.label.clone(), element_registry, asset_manager)?;
            anchor_element_id = option_button.get_main_element_id();

            let option_button_id = widget_registry.add_button(option_button);
            options.push(DropdownOptionButton{ button_id: option_button_id, value: option.value.clone(), label: option.label.clone() });
        }

        let icon_right = button.padding().right();
        let icon_widget = IconBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightInside(icon_right), button.get_main_element_id()))
            .with_color(self.text_color.clone())
            .with_z_index(self.z_index + 0.01)
            .with_height(6.0)
            .build(element_registry, asset_manager)
        ?;

        Ok(Dropdown {
            z_index: self.z_index,
            button,
            icon_widget,
            options,
            is_open: false,
            selected: selected_value,
            option_buttons_respect_draw_bounds: self.option_buttons_respect_draw_bounds,
        })
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
}

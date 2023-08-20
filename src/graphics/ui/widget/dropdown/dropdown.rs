use std::fmt::Debug;

use glam::Vec2;

use crate::{asset_manager::AssetManager, graphics::ui::{ElementRegistry, widget::{Button, ButtonBuilder, UiWidget}, interface::{is_valid_z_index, MAX_Z_INDEX}, Position, AnchorPoint}, log, input::{Input, InputAction}};

struct DropdownOptionButton<T: Debug + Clone> {
    button: Button,
    value: T,
    label: String,
}

pub struct Dropdown<T: Debug + Clone> {
    z_index: f32,
    button: Button,
    options: Vec<DropdownOptionButton<T>>,
    is_open: bool,
    selected: Option<T>,
    /// if false, option buttons do not use draw bounds
    option_buttons_respect_draw_bounds: bool,
}

// TODO implement for all types instead of only for u32
impl <T: Debug + Clone> UiWidget for Dropdown<T> {
    fn show(&self, element_registry: &mut ElementRegistry) {
        self.button.show(element_registry);

        for option in self.options.iter() {
            option.button.show(element_registry);
        }
    }

    fn hide(&self, element_registry: &mut ElementRegistry) {
        self.button.hide(element_registry);
        
        for option in self.options.iter() {
            option.button.hide(element_registry);
        }
    }

    fn anchor_element_id(&self) -> u32 {
        self.button.anchor_element_id()
    }

    fn z_index(&self) -> f32 {
        self.z_index
    }

    fn size(&self, element_registry: &ElementRegistry) -> Result<Vec2, String> {
        Ok(element_registry.get_element_size(self.anchor_element_id()).unwrap())
    }

    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry) {
        self.button.set_position(position, element_registry);
    }

    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) {
        self.z_index = z_index;
        self.button.set_z_index(z_index, element_registry);

        for option in self.options.iter_mut() {
            option.button.set_z_index(Self::option_button_z_index(z_index), element_registry);
        }
    }

    fn set_draw_bounds(&self, draw_bounds: crate::graphics::ui::draw_bounds::DrawBounds, element_registry: &mut ElementRegistry) {
        _ = self.button.set_draw_bounds(draw_bounds, element_registry);

        if self.option_buttons_respect_draw_bounds {
            for option in self.options.iter() {
                option.button.set_draw_bounds(draw_bounds, element_registry);
            }
        }
    }

    fn get_screen_position(&self, element_registry: &ElementRegistry) -> Result<Vec2, String> {
        element_registry.get_element_screen_position(self.anchor_element_id())
    }
    fn position_transform(&self, element_registry: &ElementRegistry) -> Result<Vec2, String> {
        element_registry.get_element_position_transform(self.anchor_element_id())
    }
}

impl<T: Debug + Clone> Dropdown<T> {
    pub fn new(builder: DropdownBuilder<T>, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<Self, String> {
        builder.validate()?;

        let selected_value: Option<T>;
        let label;

        if builder.placeholder_text.is_some() {
            label = builder.placeholder_text.unwrap();
            selected_value = None;
        } else if builder.initially_selected_index.is_some() {
            let selected_option = &builder.options[builder.initially_selected_index.unwrap() as usize];
            label = selected_option.label.clone();
            selected_value = Some(selected_option.value.clone());
        } else {
            label = "".to_string();
            selected_value = None;
        };

        let button = Button::new(label, ButtonBuilder::new()
            .with_position(builder.position)
            .with_z_index(builder.z_index)
        , element_registry, asset_manager)?;

        // TODO add background for options

        let mut options = vec![];
        let mut anchor_element_id = button.anchor_element_id();
        for option in builder.options {
            let option_button = Button::new(option.label.clone(), ButtonBuilder::new()
                .with_position(Position::ElementAnchor(AnchorPoint::BottomOutside(5.0), anchor_element_id))
                .with_width(button.width())
                .with_height(button.height())
                .with_mouse_action_to_activate(InputAction::UpOrDown)
                .with_hidden(true)
                .with_z_index(Self::option_button_z_index(builder.z_index))
            , element_registry, asset_manager)?;

            anchor_element_id = option_button.anchor_element_id();

            options.push(DropdownOptionButton{ button: option_button, value: option.value, label: option.label });
        }

        Ok(Self {
            z_index: builder.z_index,
            button,
            options,
            is_open: false,
            selected: selected_value,
            option_buttons_respect_draw_bounds: builder.option_buttons_respect_draw_bounds,
        })
    }

    fn option_button_z_index(base_z_index: f32) -> f32 {
        (1000.0 + base_z_index).min(MAX_Z_INDEX - 1.0)
    }

    /// Returns the newly selected value, or None if nothing has changed
    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Option<T> {
        if self.is_open {
            for option in self.options.iter() {
                if option.button.is_clicked(input, element_registry) {
                    let value = option.value.clone();

                    match element_registry.set_text(self.button.text_element_id(), &option.label, asset_manager) {
                        Ok(_) => (),
                        Err(err) => {
                            log::engine_err(format!("failed to set selected dropdown value {:?}: {}", value, err));
                            return None;
                        },
                    }
                    
                    self.is_open = false;
                    self.handle_state(element_registry);

                    return Some(value);
                }
            }
        }

        if self.button.is_clicked(input, element_registry) {
            self.is_open = !self.is_open;
            self.handle_state(element_registry);
        }

        None
    }

    pub fn handle_state(&mut self, element_registry: &mut ElementRegistry) {
        if self.is_open {
            for opt in self.options.iter() {
                opt.button.show(element_registry);
            }
        } else {
            for opt in self.options.iter() {
                opt.button.hide(element_registry);
            }
        }
    }
}

pub struct DropdownOption<T: Debug + Clone> {
    pub label: String,
    pub value: T,
}

pub struct DropdownBuilder<T: Debug + Clone> {
    placeholder_text: Option<String>,
    options: Vec<DropdownOption<T>>,
    initially_selected_index: Option<u32>, // index of the options list
    z_index: f32,
    position: Position,
    option_buttons_respect_draw_bounds: bool,
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
        }
    }

    pub fn with_placeholder_text(mut self, placeholder_text: String) -> Self {
        self.placeholder_text = Some(placeholder_text);
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
}

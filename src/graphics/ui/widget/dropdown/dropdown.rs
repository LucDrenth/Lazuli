use crate::{asset_manager::AssetManager, graphics::ui::{ElementRegistry, widget::{Button, ButtonBuilder}, interface::is_valid_z_index, Position, AnchorPoint}, log, input::{Input, InputAction}};

struct DropdownOptionButton {
    button: Button,
    value: u32,
    label: String,
}

pub struct Dropdown {
    z_index: f32,
    button: Button,
    options: Vec<DropdownOptionButton>,
    is_open: bool,
    selected: Option<u32>,
}

impl Dropdown {
    pub fn new(builder: DropdownBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<Self, String> {
        builder.validate()?;

        let label: String = if builder.placeholder_text.is_some() {
            builder.placeholder_text.unwrap()
        } else if builder.initially_selected.is_some() {
            builder.options[builder.initially_selected.unwrap() as usize].label.clone()
        } else {
            "".to_string()
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
                // TODO styling
                .with_position(Position::ElementAnchor(AnchorPoint::BottomOutside(5.0), anchor_element_id))
                .with_width(button.width())
                .with_height(button.height())
                .with_mouse_action_to_activate(InputAction::UpOrDown)
            , element_registry, asset_manager)?;

            anchor_element_id = option_button.anchor_element_id();

            // TODO add with_initially_hidden function to ButtonBuilder
            option_button.hide(element_registry);

            options.push(DropdownOptionButton{ button: option_button, value: option.value, label: option.label });
        }

        Ok(Self {
            z_index: builder.z_index,
            button,
            options,
            is_open: false,
            selected: builder.initially_selected,
        })
    }

    /// Returns the newly selected value, or None if nothing has changed
    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Option<u32> {
        if self.is_open {
            for option in self.options.iter() {
                if option.button.is_clicked(input, element_registry) {
                    let value = option.value;

                    match element_registry.set_text(self.button.text_element_id(), &option.label, asset_manager) {
                        Ok(_) => (),
                        Err(err) => {
                            log::engine_err(format!("failed to set selected dropdown value {}: {}", value, err));
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

pub struct DropdownOption {
    pub label: String,
    pub value: u32,
}

pub struct DropdownBuilder {
    placeholder_text: Option<String>,
    options: Vec<DropdownOption>,
    initially_selected: Option<u32>, // index of the options list
    z_index: f32,
    position: Position,
}

impl DropdownBuilder {
    pub fn new() -> Self {
        Self {
            placeholder_text: None,
            options: vec![],
            initially_selected: Some(0),
            z_index: 10.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
        }
    }

    pub fn with_placeholder_text(mut self, placeholder_text: String) -> Self {
        self.placeholder_text = Some(placeholder_text);
        self.initially_selected = None;
        self
    }

    pub fn with_initially_selected(mut self, initially_selected: u32) -> Self {
        self.initially_selected = Some(initially_selected);
        self.placeholder_text = None;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.initially_selected.is_some() && self.options.len() <= self.initially_selected.unwrap() as usize {
            return Err(format!(
                "Initially selected index ({}) is higher than the number of options ({})"
                , self.initially_selected.unwrap(), self.options.len()
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

    pub fn with_options(mut self, options: Vec<DropdownOption>) -> Self {
        self.options = options;
        self
    }
}

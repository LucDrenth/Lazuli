use crate::{graphics::{ui::widget::{Slider, SliderBuilder, SliderUpdateResult, Button, ButtonBuilder, UiWidget, Dropdown, DropdownBuilder}, Color}, asset_manager::AssetManager, input::Input, log};

use super::ElementRegistry;

// TODO abstractiate to WidgetEntry
struct ButtonEntry {
    button: Button,
    id: u32,
    is_clicked: bool,
}
struct SliderEntry {
    slider: Slider,
    id: u32,
    update_result: Option<SliderUpdateResult>,
}

// TODO support more types than Dropdown<u32>
struct DropdownEntry {
    dropdown: Dropdown<u32>,
    id: u32,
    update_result: Option<u32>,
}


pub struct WidgetRegistry {
    buttons: Vec<ButtonEntry>,
    sliders: Vec<SliderEntry>,
    dropdowns: Vec<DropdownEntry>,
    current_id: u32,
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            buttons: vec![],
            sliders: vec![],
            dropdowns: vec![],
            current_id: 0,
        }
    }

    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) {
        // TODO update based on z-index of both buttons and sliders together

        // update sliders
        for entry in self.sliders.iter_mut() {
            entry.update_result = entry.slider.update(input, element_registry, asset_manager);
        }

        // update buttons
        let mut is_any_button_clicked = false;
        for entry in self.buttons.iter_mut() {
            if is_any_button_clicked {
                entry.is_clicked = false;
            } else {
                entry.is_clicked = entry.button.is_clicked(input, element_registry);
                is_any_button_clicked = entry.is_clicked;
            }
        }

        // update dropdowns
        for entry in self.dropdowns.iter_mut() {
            entry.update_result = entry.dropdown.update(input, element_registry, asset_manager);
        }
    }

    pub fn add_slider(&mut self, builder: SliderBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let slider = Slider::new(builder, element_registry, asset_manager)?;
        
        self.current_id += 1;
        self.sliders.push(SliderEntry {
            slider,
            id: self.current_id,
            update_result: None,
        });
        self.sort_sliders_by_z_index();

        Ok(self.current_id)
    }

    pub fn add_button(&mut self, label: String, builder: ButtonBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let button = Button::new(label, builder, element_registry, asset_manager)?;
        
        self.current_id += 1;
        self.buttons.push(ButtonEntry {
            button,
            id: self.current_id,
            is_clicked: false,
        });
        self.sort_buttons_by_z_index();

        Ok(self.current_id)
    }

    pub fn add_dropdown(&mut self, builder: DropdownBuilder<u32>, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let dropdown = Dropdown::new(builder, element_registry, asset_manager)?;

        self.current_id += 1;
        self.dropdowns.push(DropdownEntry {
            dropdown,
            id: self.current_id,
            update_result: None,
        });
        self.sort_dropdowns_by_z_index();

        Ok(self.current_id)
    }

    // Sort elements so that the elements with the lowest z-index are at the start of the list
    fn sort_sliders_by_z_index(&mut self) {
        self.sliders.sort_by(|a, b| b.slider.z_index().total_cmp(&a.slider.z_index()));
    }
    // Sort elements so that the elements with the lowest z-index are at the start of the list
    fn sort_buttons_by_z_index(&mut self) {
        self.buttons.sort_by(|a, b| b.button.z_index().total_cmp(&a.button.z_index()));
    }
    // Sort elements so that the elements with the lowest z-index are at the start of the list
    fn sort_dropdowns_by_z_index(&mut self) {
        self.dropdowns.sort_by(|a, b| b.dropdown.z_index().total_cmp(&a.dropdown.z_index()));
    }

    pub fn slider_update_result(&self, slider_id: u32) -> Option<SliderUpdateResult> {
        for entry in self.sliders.iter() {
            if entry.id == slider_id {
                match entry.update_result {
                    Some(slider_update_result) => return Some(slider_update_result.clone()),
                    None => return None,
                }
            }
        }

        log::engine_warn(format!("WidgetRegistry.slider_update_result returned None because slider with id {} was not found", slider_id));

        None
    }

    pub fn is_button_clicked(&self, button_id: u32) -> bool {
        for entry in self.buttons.iter() {
            if entry.id == button_id {
                return entry.is_clicked
            }
        }

        log::engine_warn(format!("WidgetRegistry.is_button_clicked returned false because button with id {} was not found", button_id));
        return false;
    }

    pub fn dropdown_update_result(&self, dropdown_id: u32) -> Option<u32> {
        for entry in self.dropdowns.iter() {
            if entry.id == dropdown_id {
                return entry.update_result.clone();
            }
        }

        log::engine_warn(format!("WidgetRegistry.dropdown_update_result returned None because dropdown with id {} was not found", dropdown_id));

        None
    }

    pub fn get_anchor_element_id(&self, widget_id: u32) -> Option<u32> {
        match self.get_widget_by_id(widget_id) {
            Some(widget) => Some(widget.anchor_element_id()),
            None => {
                log::engine_warn(format!("Returning None for get_widget_by_id because widget with id {} was not found", widget_id));
                None
            },
        }
    }

    pub fn show_widget(&self, widget_id: u32, element_registry: &mut ElementRegistry) {
        self.get_widget_by_id(widget_id).unwrap().show(element_registry);
    }
    pub fn hide_widget(&self, widget_id: u32, element_registry: &mut ElementRegistry) {
        self.get_widget_by_id(widget_id).unwrap().hide(element_registry);
    }

    fn get_widget_by_id(&self, widget_id: u32) -> Option<Box<&dyn UiWidget>> {
        for widget_entry in self.sliders.iter() {
            if widget_entry.id == widget_id { return Some(Box::new(&widget_entry.slider)) }
        }

        for widget_entry in self.buttons.iter() {
            if widget_entry.id == widget_id { return Some(Box::new(&widget_entry.button)) }
        }

        for widget_entry in self.dropdowns.iter() {
            if widget_entry.id == widget_id { return Some(Box::new(&widget_entry.dropdown)) }
        }

        None
    }


    // =================================================== \\
    // ======= Functions to get individual widgets ======= \\

    fn get_slider(&self, slider_id: u32) -> Option<&Slider> {
        for entry in self.sliders.iter() {
            if entry.id == slider_id { return Some(&entry.slider) }
        }

        None
    }
    fn get_mut_slider(&mut self, slider_id: u32) -> Option<&mut Slider> {
        for entry in self.sliders.iter_mut() {
            if entry.id == slider_id { return Some(&mut entry.slider) }
        }

        None
    }
    fn get_button(&self, button_id: u32) -> Option<&Button> {
        for entry in self.buttons.iter() {
            if entry.id == button_id { return Some(&entry.button) }
        }

        None
    }
    fn get_mut_button(&mut self, button_id: u32) -> Option<&mut Button> {
        for entry in self.buttons.iter_mut() {
            if entry.id == button_id { return Some(&mut entry.button) }
        }

        None
    }
    fn get_dropdown(&self, dropdown_id: u32) -> Option<&Dropdown<u32>> {
        for entry in self.dropdowns.iter() {
            if entry.id == dropdown_id { return Some(&entry.dropdown) }
        }

        None
    }
    fn get_mut_dropdown(&mut self, dropdown_id: u32) -> Option<&mut Dropdown<u32>> {
        for entry in self.dropdowns.iter_mut() {
            if entry.id == dropdown_id { return Some(&mut entry.dropdown) }
        }

        None
    }

    // =================================================== \\
    // ============ Widget specific functions ============ \\

    pub fn set_slider_value(&mut self, value: f32, slider_id: u32, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) {
        match self.get_mut_slider(slider_id) {
            Some(slider) => slider.set_value(value, element_registry, asset_manager),
            None => log::engine_warn(format!("Failed to set slider value because slider with id {} was not found", slider_id)),
        }
    }

    pub fn set_button_background_color(&self, color: Color, button_id: u32, element_registry: &mut ElementRegistry) -> Result<(), String> {
        match self.get_button(button_id) {
            Some(button) => button.set_background_color(color, element_registry),
            None => Err( format!("Failed to set button background color because button with id {} was not found", button_id) ),
        }
    }

    pub fn set_button_text_color(&self, color: Color, button_id: u32, element_registry: &mut ElementRegistry) -> Result<(), String> {
        match self.get_button(button_id) {
            Some(button) => button.set_text_color(color, element_registry),
            None => Err( format!("Failed to set button text color because button with id {} was not found", button_id) ),
        }
    }
}

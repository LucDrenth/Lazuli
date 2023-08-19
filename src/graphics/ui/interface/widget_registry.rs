use glam::Vec2;

use crate::{graphics::{ui::{widget::{Slider, SliderBuilder, SliderUpdateResult, Button, ButtonBuilder, UiWidget, Dropdown, DropdownBuilder}, Position, draw_bounds::DrawBounds}, Color}, asset_manager::AssetManager, input::Input, log};

use super::{ElementRegistry, widget_list::WidgetList};

pub struct WidgetRegistry {
    buttons: WidgetList<Button, bool>,
    sliders: WidgetList<Slider, Option<SliderUpdateResult>>,
    dropdowns: WidgetList<Dropdown<u32>, Option<u32>>, // TODO support more types than Dropdown<u32>
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            buttons: WidgetList::new(false),
            sliders: WidgetList::new(None),
            dropdowns: WidgetList::new(None),
        }
    }

    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) {
        // TODO update based on z-index of both buttons and sliders together

        // update sliders
        for entry in self.sliders.entries.iter_mut() {
            entry.update_result = entry.widget.update(input, element_registry, asset_manager);
        }

        // update buttons
        let mut is_any_button_clicked = false;
        for entry in self.buttons.entries.iter_mut() {
            if is_any_button_clicked {
                entry.update_result = false;
            } else {
                entry.update_result = entry.widget.is_clicked(input, element_registry);
                is_any_button_clicked = entry.update_result;
            }
        }

        // update dropdowns
        for entry in self.dropdowns.entries.iter_mut() {
            entry.update_result = entry.widget.update(input, element_registry, asset_manager);
        }
    }


    // =============================================================================== \\
    // =========== Functions for adding widgets and getting update result ============ \\

    pub fn add_slider(&mut self, builder: SliderBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let slider = Slider::new(builder, element_registry, asset_manager)?;
        Ok(self.sliders.push(slider, None))
    }
    pub fn add_button(&mut self, label: String, builder: ButtonBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let button = Button::new(label, builder, element_registry, asset_manager)?;

        Ok(self.buttons.push(button, false))
    }
    pub fn add_dropdown(&mut self, builder: DropdownBuilder<u32>, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let dropdown = Dropdown::new(builder, element_registry, asset_manager)?;
        Ok(self.dropdowns.push(dropdown, None))
    }

    pub fn slider_update_result(&self, slider_id: u32) -> Option<SliderUpdateResult> {
        self.sliders.get_update_result(slider_id)
    }
    pub fn is_button_clicked(&self, button_id: u32) -> bool {
        self.buttons.get_update_result(button_id)
    }
    pub fn dropdown_update_result(&self, dropdown_id: u32) -> Option<u32> {
        self.dropdowns.get_update_result(dropdown_id)
    }


    // =================================================== \\
    // =========== General UiWidget functions ============ \\

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

    pub fn set_widget_position(&self, widget_id: u32, position: Position, element_registry: &mut ElementRegistry) {
        self.get_widget_by_id(widget_id).unwrap().set_position(position, element_registry);
    }
    pub fn set_widget_z_index(&mut self, widget_id: u32, z_index: f32, element_registry: &mut ElementRegistry) {
        self.get_mut_widget_by_id(widget_id).unwrap().set_z_index(z_index, element_registry);
    }
    pub fn set_widget_draw_bounds(&mut self, widget_id: u32, draw_bounds: DrawBounds, element_registry: &mut ElementRegistry) {
        self.get_mut_widget_by_id(widget_id).unwrap().set_draw_bounds(draw_bounds, element_registry);
    }

    fn get_widget_by_id(&self, widget_id: u32) -> Option<Box<&dyn UiWidget>> {
        for widget_entry in self.sliders.entries.iter() {
            if widget_entry.id == widget_id { return Some(Box::new(&widget_entry.widget)) }
        }

        for widget_entry in self.buttons.entries.iter() {
            if widget_entry.id == widget_id { return Some(Box::new(&widget_entry.widget)) }
        }

        for widget_entry in self.dropdowns.entries.iter() {
            if widget_entry.id == widget_id { return Some(Box::new(&widget_entry.widget)) }
        }

        None
    }

    fn get_mut_widget_by_id(&mut self, widget_id: u32) -> Option<Box<&mut dyn UiWidget>> {
        for widget_entry in self.sliders.entries.iter_mut() {
            if widget_entry.id == widget_id { return Some(Box::new(&mut widget_entry.widget)) }
        }

        for widget_entry in self.buttons.entries.iter_mut() {
            if widget_entry.id == widget_id { return Some(Box::new(&mut widget_entry.widget)) }
        }

        for widget_entry in self.dropdowns.entries.iter_mut() {
            if widget_entry.id == widget_id { return Some(Box::new(&mut widget_entry.widget)) }
        }

        None
    }

    pub fn get_widget_size(&self, widget_id: u32, element_registry: &ElementRegistry) -> Result<Vec2, String> {
        match self.get_widget_by_id(widget_id) {
            Some(widget) => widget.size(element_registry),
            None => Err(format!("widget with id {} not found", widget_id)),
        }
    }


    // =================================================== \\
    // ======= Functions to get individual widgets ======= \\

    fn get_slider(&self, slider_id: u32) -> Option<&Slider> {
        self.sliders.get_widget(slider_id)
    }
    fn get_mut_slider(&mut self, slider_id: u32) -> Option<&mut Slider> {
        self.sliders.get_mut_widget(slider_id)

    }
    fn get_button(&self, button_id: u32) -> Option<&Button> {
        self.buttons.get_widget(button_id)
    }
    fn get_mut_button(&mut self, button_id: u32) -> Option<&mut Button> {
        self.buttons.get_mut_widget(button_id)
    }
    fn get_dropdown(&self, dropdown_id: u32) -> Option<&Dropdown<u32>> {
        self.dropdowns.get_widget(dropdown_id)

    }
    fn get_mut_dropdown(&mut self, dropdown_id: u32) -> Option<&mut Dropdown<u32>> {
        self.dropdowns.get_mut_widget(dropdown_id)
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

use glam::Vec2;

use crate::{graphics::{ui::{widget::{Slider, SliderBuilder, SliderUpdateResult, Button, ButtonBuilder, UiWidget, Dropdown, DropdownBuilder, Icon, IconBuilder}, UiWidgetId, bounds_2d::Bounds2d, Position, UiElementId, UiUpdateTargets, WidgetUpdateTarget, UpdateTargetCollection}, Color}, asset_manager::AssetManager, input::Input, log, ResourceId};

use super::{ElementRegistry, widget_list::WidgetList, LayoutRegistry};

pub struct WidgetRegistryUdpateResult {
    pub update_targets_visibility: UiUpdateTargets<bool>,
    pub buttons_to_change_text: Vec<WidgetUpdateTarget<String>>,
    pub widgets_to_set_main_element_custom_shader_value_f32: Vec<WidgetUpdateTarget<(String, f32)>>,
}
impl Default for WidgetRegistryUdpateResult {
    fn default() -> Self {
        Self { 
            update_targets_visibility: Default::default(), 
            buttons_to_change_text: vec![],
            widgets_to_set_main_element_custom_shader_value_f32: vec![],
        }
    }
}

pub struct WidgetRegistry {
    buttons: WidgetList<Button, bool>,
    sliders: WidgetList<Slider, Option<SliderUpdateResult>>,
    dropdowns: WidgetList<Dropdown<u32>, Option<u32>>, // TODO support more types than Dropdown<u32>
    icons: WidgetList<Icon, ()>,
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            buttons: WidgetList::new(false),
            sliders: WidgetList::new(None),
            dropdowns: WidgetList::new(None),
            icons: WidgetList::new(()),
        }
    }

    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) -> WidgetRegistryUdpateResult {
        let mut result: WidgetRegistryUdpateResult = Default::default();

        // update sliders
        for entry in self.sliders.entries.iter_mut() {
            entry.update_result = entry.widget.update(input, element_registry, asset_manager);
        }

        // update buttons
        let mut clicked_button_id: Option<ResourceId<UiWidgetId>> = None;
        for entry in self.buttons.entries.iter_mut() {
            if clicked_button_id.is_some() {
                entry.update_result = false;
            } else {
                entry.update_result = entry.widget.is_clicked(element_registry);
                if entry.update_result {
                    clicked_button_id = Some(entry.id);
                }
            }
        }

        // update dropdowns
        for entry in self.dropdowns.entries.iter_mut() {
            entry.update_result = entry.widget.update(&clicked_button_id, &mut result);
        }

        result
    }

    pub fn get_widget_main_element_id(&self, widget_id: &ResourceId<UiWidgetId>) -> Option<ResourceId<UiElementId>> {
        match self.get_widget_by_id(widget_id) {
            Some(widget) => Some(widget.get_main_element_id()),
            None => None,
        }
    }
    pub fn get_widget_size(&self, widget_id: &ResourceId<UiWidgetId>, element_registry: &ElementRegistry) -> Result<Vec2, String> {
        let main_element_id = self.get_widget_main_element_id(widget_id).unwrap();
        element_registry.get_element_size(&main_element_id)
    }
    pub fn get_widget_screen_position(&self, widget_id: &ResourceId<UiWidgetId>, element_registry: &ElementRegistry) -> Result<Vec2, String> {
        let main_element_id = self.get_widget_main_element_id(widget_id).unwrap();
        element_registry.get_element_screen_position(&main_element_id)
    }
    pub fn get_widget_position_transform(&self, widget_id: &ResourceId<UiWidgetId>, element_registry: &ElementRegistry) -> Result<Vec2, String> {
        let main_element_id = self.get_widget_main_element_id(widget_id).unwrap();
        element_registry.get_element_position_transform(&main_element_id)
    }


    // ===================================================================================== \\
    // ========== Methods for updating a widget (and its embedded widgets/layouts) ========== \\

    pub fn set_widget_z_index(&mut self, widget_id: &ResourceId<UiWidgetId>, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.get_mut_widget_by_id(widget_id).unwrap().set_z_index(z_index, element_registry)
    }
    pub fn set_widget_width(&mut self, widget_id: &ResourceId<UiWidgetId>, width: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.get_mut_widget_by_id(widget_id).unwrap().set_width(width, element_registry)
    }
    pub fn set_widget_height(&mut self, widget_id: &ResourceId<UiWidgetId>, height: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.get_mut_widget_by_id(widget_id).unwrap().set_height(height, element_registry)
    }
    pub fn set_widget_size(&mut self, widget_id: &ResourceId<UiWidgetId>, size: Vec2, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2> {
        self.get_mut_widget_by_id(widget_id).unwrap().set_size(size, element_registry)
    }
    pub fn set_widget_draw_bounds(&mut self, widget_id: &ResourceId<UiWidgetId>, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d> {
        self.get_mut_widget_by_id(widget_id).unwrap().set_draw_bounds(draw_bounds, element_registry)
    }
    pub fn set_widget_position(&mut self, widget_id: &ResourceId<UiWidgetId>, position: Position, element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        self.get_mut_widget_by_id(widget_id).unwrap().set_position(position, element_registry)
    }
    pub fn set_widget_visibility(&mut self, widget_id: &ResourceId<UiWidgetId>, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        self.get_mut_widget_by_id(widget_id).unwrap().set_visibility(visible, element_registry)
    }
    

    // =============================================================================== \\
    // ============= Methods for adding widgets and getting update result ============= \\

    pub fn add_slider(&mut self, slider: Slider) -> ResourceId<UiWidgetId> {
        self.sliders.push(slider)
    }
    pub fn create_slider(&mut self, builder: &SliderBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        let slider = builder.build(element_registry, asset_manager)?;
        Ok(self.sliders.push(slider))
    }

    pub fn add_button(&mut self, button: Button) -> ResourceId<UiWidgetId> {
        self.buttons.push(button)
    }
    pub fn create_button(&mut self, label: impl Into<String>, builder: &ButtonBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        let button = builder.build(label, element_registry, asset_manager)?;
        Ok(self.buttons.push(button))
    }

    pub fn add_dropdown(&mut self, dropdown: Dropdown<u32>) -> ResourceId<UiWidgetId> {
        self.dropdowns.push(dropdown)
    }
    pub fn create_dropdown(&mut self, builder: &DropdownBuilder<u32>, element_registry: &mut ElementRegistry, layout_registry: &mut LayoutRegistry, asset_manager: &mut dyn AssetManager) -> Result<(ResourceId<UiWidgetId>, Vec<UpdateTargetCollection>), String> {
        let (dropdown, update_targets) = builder.build(element_registry, self, layout_registry, asset_manager)?;
        let id = self.dropdowns.push(dropdown);

        Ok((id, update_targets))
    }

    pub fn add_icon(&mut self, icon: Icon) -> ResourceId<UiWidgetId> {
        self.icons.push(icon)
    }
    pub fn create_icon(&mut self, builder: &IconBuilder, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        let icon = builder.build(element_registry, asset_manager)?;
        Ok(self.icons.push(icon))
    }


    pub fn slider_update_result(&self, slider_id: &ResourceId<UiWidgetId>) -> Option<SliderUpdateResult> {
        self.sliders.get_update_result(slider_id)
    }
    pub fn is_button_clicked(&self, button_id: &ResourceId<UiWidgetId>) -> bool {
        self.buttons.get_update_result(button_id)
    }
    pub fn dropdown_update_result(&self, dropdown_id: &ResourceId<UiWidgetId>) -> Option<u32> {
        self.dropdowns.get_update_result(dropdown_id)
    }


    // ================================================== \\
    // ============ Methods to get a UiWidget ============ \\

    pub fn get_widget_by_id(&self, widget_id: &ResourceId<UiWidgetId>) -> Option<Box<&dyn UiWidget>> {
        macro_rules! get_widget {
            ($list:expr) => {
                if let Some(widget) = $list.get_widget(widget_id) {
                    return Some(Box::new(widget));
                }
            };
        }
    
        get_widget!(self.buttons);
        get_widget!(self.sliders);
        get_widget!(self.dropdowns);
        get_widget!(self.icons);
    
        None
    }

    pub fn get_mut_widget_by_id(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<Box<&mut dyn UiWidget>> {
        macro_rules! get_widget {
            ($list:expr) => {
                if let Some(widget) = $list.get_mut_widget(widget_id) {
                    return Some(Box::new(widget));
                }
            };
        }

        get_widget!(self.buttons);
        get_widget!(self.sliders);
        get_widget!(self.dropdowns);
        get_widget!(self.icons);

        None
    }


    // =================================================== \\
    // ======== Methods to get individual widgets ======== \\

    pub fn get_slider(&self, slider_id: &ResourceId<UiWidgetId>) -> Option<&Slider> {
        self.sliders.get_widget(slider_id)
    }
    pub fn get_mut_slider(&mut self, slider_id: &ResourceId<UiWidgetId>) -> Option<&mut Slider> {
        self.sliders.get_mut_widget(slider_id)

    }
    pub fn get_button(&self, button_id: &ResourceId<UiWidgetId>) -> Option<&Button> {
        self.buttons.get_widget(button_id)
    }
    pub fn get_mut_button(&mut self, button_id: &ResourceId<UiWidgetId>) -> Option<&mut Button> {
        self.buttons.get_mut_widget(button_id)
    }
    pub fn get_dropdown(&self, dropdown_id: &ResourceId<UiWidgetId>) -> Option<&Dropdown<u32>> {
        self.dropdowns.get_widget(dropdown_id)
    }
    pub fn get_mut_dropdown(&mut self, dropdown_id: &ResourceId<UiWidgetId>) -> Option<&mut Dropdown<u32>> {
        self.dropdowns.get_mut_widget(dropdown_id)
    }
    pub fn get_icon(&self, icon_id: &ResourceId<UiWidgetId>) -> Option<&Icon> {
        self.icons.get_widget(icon_id)
    }
    pub fn get_mut_icon(&mut self, icon_id: &ResourceId<UiWidgetId>) -> Option<&mut Icon> {
        self.icons.get_mut_widget(icon_id)
    }


    // ================================================== \\
    // ============= Widget specific methods ============= \\

    pub fn set_slider_value(&mut self, value: f32, slider_id: &ResourceId<UiWidgetId>, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) {
        match self.get_mut_slider(slider_id) {
            Some(slider) => slider.set_value(value, element_registry, asset_manager),
            None => log::engine_warn( format!("Failed to set slider value because slider with id {} was not found", slider_id.id()) ),
        }
    }

    pub fn set_button_background_color(&self, color: Color, button_id: &ResourceId<UiWidgetId>, element_registry: &mut ElementRegistry) -> Result<(), String> {
        match self.get_button(button_id) {
            Some(button) => button.set_background_color(color, element_registry),
            None => Err( format!("Failed to set button background color because button with id {} was not found", button_id.id()) ),
        }
    }

    pub fn set_button_text_color(&self, color: Color, button_id: &ResourceId<UiWidgetId>, element_registry: &mut ElementRegistry) -> Result<(), String> {
        match self.get_button(button_id) {
            Some(button) => button.set_text_color(color, element_registry),
            None => Err( format!("Failed to set button text color because button with id {} was not found", button_id.id()) ),
        }
    }

    // ================================================== \\
    // ============ Methods to remove widgets ============ \\

    pub fn remove_widget(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<Box<dyn UiWidget>> {
        self.remove_button(widget_id).or_else(||{ 
        self.remove_slider(widget_id).or_else(||{
        self.remove_dropdown(widget_id).or_else(||{
        self.remove_icon(widget_id)
        }) }) })
    }

    pub fn remove_button(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<Box<dyn UiWidget>> {
        self.buttons.remove(widget_id).map(|widget| Box::new(widget) as Box<dyn UiWidget>)
    }
    pub fn remove_slider(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<Box<dyn UiWidget>> {
        self.sliders.remove(widget_id).map(|widget| Box::new(widget) as Box<dyn UiWidget>)
    }
    pub fn remove_dropdown(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<Box<dyn UiWidget>> {
        self.dropdowns.remove(widget_id).map(|widget| Box::new(widget) as Box<dyn UiWidget>)
    }
    pub fn remove_icon(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<Box<dyn UiWidget>> {
        self.icons.remove(widget_id).map(|widget| Box::new(widget) as Box<dyn UiWidget>)
    }
}

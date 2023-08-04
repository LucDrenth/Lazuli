use crate::{graphics::ui::widget::{Slider, SliderBuilder, SliderUpdateResult}, asset_registry::AssetRegistry, input::Input, log};

use super::ElementRegistry;

struct SliderEntry {
    slider: Slider,
    id: u32,
    update_result: Option<SliderUpdateResult>
}


pub struct WidgetRegistry {
    sliders: Vec<SliderEntry>,
    slider_id: u32,
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            sliders: vec![],
            slider_id: 0,
        }
    }

    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) {
        for entry in self.sliders.iter_mut() {
            entry.update_result = entry.slider.update(input, element_registry, asset_registry);
        }
    }

    pub fn add_slider(&mut self, builder: SliderBuilder, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        let slider = Slider::new(builder, element_registry, asset_registry)?;
        
        self.slider_id += 1;
        self.sliders.push(SliderEntry {
            slider,
            id: self.slider_id,
            update_result: None,
        });
        self.sort_sliders_by_z_index();

        Ok(self.slider_id)
    }

    // Sort elements so that the elements with the lowest z-index are at the start of the list
    fn sort_sliders_by_z_index(&mut self) {
        self.sliders.sort_by(|a, b| b.slider.z_index().total_cmp(&a.slider.z_index()));
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

        log::engine_warn(format!("Returning None for slider_update_result because slider with id {} was not found", slider_id));

        None
    }

    pub fn slider_anchor_element_id(&self, slider_id: u32) -> Option<u32> {
        match self.get_slider(slider_id) {
            Some(slider) => {
                Some(slider.anchor_element_id())
            },
            None => {
                log::engine_warn(format!("Returning None for slider_anchor_element_id because slider with id {} was not found", slider_id));
                None
            },
        }
    }

    pub fn set_slider_value(&mut self, value: f32, slider_id: u32, element_registry: &mut ElementRegistry, asset_registry: &mut AssetRegistry) {
        match self.get_mut_slider(slider_id) {
            Some(slider) => slider.set_value(value, element_registry, asset_registry),
            None => log::engine_warn(format!("Failed to set slider value because slider with id {} was not found", slider_id)),
        }
    }

    fn get_slider(&self, slider_id: u32) -> Option<&Slider> {
        for entry in self.sliders.iter() {
            if entry.id == slider_id {
                return Some(&entry.slider);
            }
        }

        None
    }

    fn get_mut_slider(&mut self, slider_id: u32) -> Option<&mut Slider> {
        for entry in self.sliders.iter_mut() {
            if entry.id == slider_id {
                return Some(&mut entry.slider);
            }
        }

        None
    }
}

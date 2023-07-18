use std::collections::HashMap;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, lz_core_err, asset_registry::AssetRegistry};

use super::{ui_element::UiElement, TextBuilder, Text};

pub struct Interface {
    window_resize_listener: EventReader<WindowResizeEvent>,

    elements: HashMap<u32, Box<dyn UiElement>>,
    current_element_id: u32,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem) -> Self {
        let window_resize_listener = event_system.register::<WindowResizeEvent>();

        Self {
            window_resize_listener,
            elements: HashMap::new(),
            current_element_id: 0,
        }
    }

    pub fn update(&mut self, asset_registry: &mut AssetRegistry) {
        match self.window_resize_listener.read().last() {
            Some(e) => {
                // update view uniform of all ui elements
                for (_, ui_element) in self.elements.iter() {
                    let shader_id;

                    match asset_registry.get_material_by_id(ui_element.material_id()) {
                        Some(material) => {
                            shader_id = material.shader_id;
                        }
                        None => continue,
                    }

                    asset_registry.get_shader_by_id(shader_id).unwrap().set_uniform("view", for_shader(e.width as f32, e.height as f32));
                }
            },
            None => (),            
        }
    }

    pub fn draw(&self, asset_registry: &mut AssetRegistry) {
        for (_, ui_element) in self.elements.iter() {
            ui_element.draw(asset_registry);
        }
    }

    pub fn add_element(&mut self, element: Box<dyn UiElement>) -> u32 {
        self.current_element_id += 1;

        match self.elements.entry(self.current_element_id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                lz_core_err!("Encountered duplicate id [{}] while adding interface font", self.current_element_id);
                return 0;
            },
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(element);
                return self.current_element_id;
            }
        }
    }

    pub fn add_text(&mut self, text: String, font_id: u32, text_builder: &TextBuilder, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        let text = Text::new(text, font_id, text_builder, asset_registry)?;
        Ok(self.add_element(Box::new(text)))
    }
}

fn for_shader(window_width: f32, window_height: f32) -> (f32, f32) {
    (1.0 / window_width, 1.0 / window_height)
}

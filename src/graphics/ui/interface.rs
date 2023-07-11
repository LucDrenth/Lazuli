use std::collections::HashMap;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, graphics::font::Font, lz_core_err};

use super::ui_element::UiElement;

pub struct Interface {
    window_resize_listener: EventReader<WindowResizeEvent>,

    fonts: HashMap<u16, Font>,
    current_font_id: u16,

    elements: HashMap<u32, Box<dyn UiElement>>,
    current_element_id: u32,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem) -> Self {
        let window_resize_listener = event_system.register::<WindowResizeEvent>();

        Self {
            window_resize_listener,
            fonts: HashMap::new(),
            current_font_id: 0,
            elements: HashMap::new(),
            current_element_id: 0,
        }
    }

    pub fn update(&mut self) {
        match self.window_resize_listener.read().last() {
            Some(e) => {
                // update view uniform of all ui elements
                for (_, ui_element) in self.elements.iter() {
                    match ui_element.material(&self) {
                        Some(material) => {
                            material.shader_program.set_uniform("view", for_shader(e.width as f32, e.height as f32));
                        }
                        None => (),
                    }
                }
            },
            None => (),            
        }
    }

    pub fn draw(&self) {
        for (_, ui_element) in self.elements.iter() {
            match ui_element.material(&self) {
                Some(material) => {
                    ui_element.draw(material);
                }
                None => (),
            }
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

    pub fn add_font(&mut self, font: Font) -> u16 {   
        self.current_font_id += 1;

        match self.fonts.entry(self.current_font_id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                lz_core_err!("Encountered duplicate id [{}] while adding interface font", self.current_font_id);
                return 0;
            },
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(font);
                return self.current_font_id;
            }
        }
    }

    pub fn get_font(&self, id: u16) -> Option<&Font> {
        self.fonts.get(&id)
    }
}

fn for_shader(window_width: f32, window_height: f32) -> (f32, f32) {
    (1.0 / window_width, 1.0 / window_height)
}

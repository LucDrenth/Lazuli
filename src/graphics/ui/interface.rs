use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, asset_registry::AssetRegistry};

use super::{ui_element::UiElement, TextBuilder, Text};

struct ElementEntry {
    id: u32,
    element: Box<dyn UiElement>,
}

pub struct Interface {
    window_resize_listener: EventReader<WindowResizeEvent>,

    elements: Vec<ElementEntry>,
    current_element_id: u32,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem) -> Self {
        let window_resize_listener = event_system.register::<WindowResizeEvent>();

        Self {
            window_resize_listener,
            elements: vec![],
            current_element_id: 0,
        }
    }

    pub fn update(&mut self, asset_registry: &mut AssetRegistry) {
        self.handle_window_resize(asset_registry);
    }

    fn handle_window_resize(&mut self, asset_registry: &mut AssetRegistry) {
        match self.window_resize_listener.read().last() {
            Some(e) => {
                // update view uniform of all ui elements
                for ui_element in self.elements.iter() {
                    let shader_id;

                    match asset_registry.get_material_by_id(ui_element.element.material_id()) {
                        Some(material) => {
                            shader_id = material.shader_id;
                        }
                        None => continue,
                    }

                    asset_registry.get_shader_by_id(shader_id).unwrap().set_uniform("view", to_view_uniform(e.width as f32, e.height as f32));
                }
            },
            None => (),            
        }
    }

    pub fn draw(&self, asset_registry: &mut AssetRegistry) {
        for ui_element in self.elements.iter() {
            ui_element.element.draw(asset_registry);
        }
    }

    pub fn add_element(&mut self, element: impl UiElement + 'static) -> u32 {
        self.current_element_id += 1;

        let new_element = ElementEntry { 
            id: self.current_element_id, 
            element: Box::new(element),
        };

        self.elements.push(new_element);
        self.sort_elements_by_z_index();

        self.current_element_id
    }

    // Sort elements so that the elements with the highest z-index are at the start of the list
    fn sort_elements_by_z_index(&mut self) {
        self.elements.sort_by(|a, b| b.element.get_z_index().total_cmp(&a.element.get_z_index()));
    }

    pub fn add_text(&mut self, text: String, font_id: u32, text_builder: &TextBuilder, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        let text = Text::new(text, font_id, text_builder, asset_registry)?;
        Ok(self.add_element(text))
    }
}

fn to_view_uniform(window_width: f32, window_height: f32) -> (f32, f32) {
    (1.0 / window_width, 1.0 / window_height)
}

use glam::Vec2;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, asset_registry::{AssetRegistry, AssetId}, input::{Input, MouseButton}, graphics::font::Font, log};

use super::{TextBuilder, Text, shapes::{Rectangle, RectangleBuilder}, element::ui_element::UiElement};

const MIN_Z_INDEX: f32 = 1.0;
const MAX_Z_INDEX: f32 = 10_000.0;

struct ElementEntry {
    id: u32,
    element: Box<dyn UiElement>,
}

pub struct Interface {
    window_resize_listener: EventReader<WindowResizeEvent>,
    size: Vec2,

    elements: Vec<ElementEntry>,
    current_element_id: u32,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem, window_size: Vec2) -> Self {
        let window_resize_listener = event_system.register::<WindowResizeEvent>();

        Self {
            window_resize_listener,
            elements: vec![],
            current_element_id: 0,
            size: window_size,
        }
    }

    pub fn update(&mut self, asset_registry: &mut AssetRegistry) {
        self.handle_window_resize(asset_registry);
    }

    fn handle_window_resize(&mut self, asset_registry: &mut AssetRegistry) {
        match self.window_resize_listener.read().last() {
            Some(e) => {
                self.size = Vec2::new(e.width as f32, e.height as f32);

                // update view uniform of all ui elements
                for element_entry in self.elements.iter_mut() {
                    element_entry.element.handle_window_resize(&self.size);
                    let shader_id;

                    match asset_registry.get_material_by_id(element_entry.element.material_id()) {
                        Some(material) => {
                            shader_id = material.shader_id.duplicate();
                        }
                        None => continue,
                    }

                    asset_registry.get_shader_by_id(&shader_id).unwrap().set_uniform("view", to_view_uniform(e.width as f32, e.height as f32));
                }
            },
            None => (),            
        }
    }

    pub fn draw(&self, asset_registry: &mut AssetRegistry) {
        for element_entry in self.elements.iter() {
            element_entry.element.draw(asset_registry);
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
        self.elements.sort_by(|a, b| a.element.world_data().z_index().total_cmp(&b.element.world_data().z_index()));
    }

    pub fn add_text(&mut self, text: String, font_id: &AssetId<Font>, text_builder: TextBuilder, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        let text = Text::new(text, font_id, text_builder, asset_registry, &self.size)?;
        Ok(self.add_element(text))
    }

    pub fn add_rectangle(&mut self, builder: RectangleBuilder, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        let rectangle = Rectangle::new(builder, asset_registry, &self.size)?;
        Ok(self.add_element(rectangle))
    }

    pub fn is_element_hovered(&self, element_id: u32, input: &Input) -> bool {
        match self.get_element(element_id) {
            Some(element) => {
                element.world_data().is_within(self.map_mouse_position(&input))
            }
            None => {
                log::engine_warn(format!("interface is_element_hovered for element {} returned false because element was not found", element_id));
                false
            }
        }
    }

    pub fn is_element_clicked(&self, element_id: u32, input: &Input) -> bool {
        return input.is_mouse_button_down(MouseButton::Left) 
            && self.is_element_hovered(element_id, input)
    }

    fn get_element(&self, element_id: u32) -> Option<&Box<dyn UiElement>> {
        for element_entry in self.elements.iter() {
            if element_entry.id == element_id {
                return Some(&element_entry.element)
            }
        }

        None
    }

    fn get_mut_element(&mut self, element_id: u32) -> Option<&mut Box<dyn UiElement>> {
        for element_entry in self.elements.iter_mut() {
            if element_entry.id == element_id {
                return Some(&mut element_entry.element)
            }
        }

        None
    }

    pub fn center_element_at_element(&mut self, element_to_center: u32, element_to_center_at: u32) {
        let target;
        match self.get_element(element_to_center_at) {
            Some(element) => target = element.world_data().clone(),
            None => todo!(),
        }

        let window_size = self.size.clone();

        match self.get_mut_element(element_to_center) {
            Some(element) => element.center_at(&target, &window_size),
            None => {
                log::engine_warn(format!("failed to center interface element at another element because element_to_center (id={}) was not found", element_to_center));
            },
        }
    }

    // map mouse position so that (0, 0) is the center
    fn map_mouse_position(&self, input: &Input) -> Vec2 {
        Vec2 {
            x: input.get_mouse_position_x() as f32 - self.size.x / 2.0,
            y: -(input.get_mouse_position_y() as f32 - self.size.y / 2.0),
        }
    }

    pub fn size(&self) -> &Vec2 { &self.size }
    pub fn width(&self) -> f32 { self.size.x }
    pub fn height(&self) -> f32 { self.size.y }
}

fn to_view_uniform(window_width: f32, window_height: f32) -> (f32, f32) {
    (2.0 / window_width, 2.0 / window_height) // 0.5 because the world coordinates for the UI range from (-size / 2) to (size / 2)
}

pub fn is_valid_z_index(z: f32) -> bool {
    z >= MIN_Z_INDEX && z <= MAX_Z_INDEX
}

// Map z index to a value between -1 and 1. 
// Actual result ranges from -0.999 to 0.999, because <= 1 and >= 1 gets culled
// A high z_index results in a low value, so it gets displayed on top of elements with a low z_index.
pub fn map_z_index_for_shader(z_index: f32) -> f32 {
    -0.999 + 1.998 * ((MAX_Z_INDEX + MIN_Z_INDEX - z_index) - MIN_Z_INDEX) / (MAX_Z_INDEX - MIN_Z_INDEX)
}

use glam::Vec2;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, asset_registry::{AssetRegistry, AssetId}, input::{Input, MouseButton}, graphics::font::{Font, PlainBitmapBuilder}, log};

use super::{TextBuilder, Text, shapes::{Rectangle, RectangleBuilder}, element::{ui_element::UiElement, AnchorElementData}};

const MIN_Z_INDEX: f32 = 1.0;
const MAX_Z_INDEX: f32 = 10_000.0;

struct ElementEntry {
    id: u32,
    element: Box<dyn UiElement>,
}

pub struct Interface {
    window_resize_listener: EventReader<WindowResizeEvent>,
    size: Vec2,
    dragged_element_id: Option<u32>, // element that is currently being dragged. Will be set to None on left mouse button up

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
            dragged_element_id: None,
        }
    }

    pub fn update(&mut self, asset_registry: &mut AssetRegistry, input: &Input) {
        if input.is_mouse_button_up(MouseButton::Left) {
            self.dragged_element_id = None;
        }

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

    pub fn generate_element_id(&mut self) -> u32 {
        self.current_element_id += 1;
        self.current_element_id
    }

    // Sort elements so that the elements with the highest z-index are at the start of the list
    fn sort_elements_by_z_index(&mut self) {
        self.elements.sort_by(|a, b| a.element.world_data().z_index().total_cmp(&b.element.world_data().z_index()));
    }

    pub fn add_text(&mut self, text: String, font_id: Option<&AssetId<Font>>, text_builder: TextBuilder, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        let font_id_to_use = match font_id {
            Some(id) => id.duplicate(),
            None => self.default_font(asset_registry)?,
        };

        let text = Text::new(text, &font_id_to_use, text_builder, asset_registry, &self)?;
        Ok(self.add_element(text))
    }

    pub fn add_rectangle(&mut self, builder: RectangleBuilder, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        let rectangle = Rectangle::new(builder, asset_registry, &self)?;
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
            None => {
                log::engine_warn(format!("failed to center interface element at another element because target (id={}) was not found", element_to_center_at));
                return;
            },
        }

        let window_size = self.size.clone();

        match self.get_mut_element(element_to_center) {
            Some(element) => element.center_at(&target, &window_size),
            None => {
                log::engine_warn(format!("failed to center interface element at another element because element_to_center (id={}) was not found", element_to_center));
            },
        }
    }

    pub fn get_element_scale(&self, element_id: u32) -> Result<Vec2, String> {
        match self.get_element(element_id) {
            Some(element) => Ok(element.get_scale()),
            None => Err(format!("failed to get scale because element with id {} was not found", element_id)),
        }
    }

    pub fn set_element_scale(&mut self, element_id: u32, scale: Vec2) -> Result<(), String> {
        let window_size = self.size().clone();

        let anchor_element_data = self.get_anchor_element_data(element_id)?;

        match self.get_mut_element(element_id) {
            Some(element) => {
                element.set_scale(scale, window_size, anchor_element_data);
                Ok(())
            },
            None => Err(format!("failed to set scale because element with id {} was not found", element_id)),
        }
    }

    /// Get anchor element data of the anchor element of the given element
    fn get_anchor_element_data(&self, element_id: u32) -> Result<Option<AnchorElementData>, String> {
        match self.get_anchor_element_id(element_id)? {
            Some(anchor_element_id) => {
                Ok(Some(AnchorElementData{
                    id: anchor_element_id,
                    size: self.get_element_size(anchor_element_id)?,
                    coordinates: self.get_element_screen_position(anchor_element_id)?,
                }))
            },
            None => Ok(None),
        }
    }

    /// Get the anchor element data of the given element
    pub fn get_anchor_data(&self, anchor_element_id: u32) -> Result<AnchorElementData, String> {
        Ok(AnchorElementData{
            id: anchor_element_id,
            size: self.get_element_size(anchor_element_id)?,
            coordinates: self.get_element_screen_position(anchor_element_id)?,
        })
    }

    pub fn get_anchor_element_id(&self, element_id: u32) -> Result<Option<u32>, String> {
        match self.get_element(element_id) {
            Some(element) => {
                Ok(element.world_data().position_type().get_anchor_element_id())
            },
            None => Err(format!("failed to get anchor element id because element with id {} was not found", element_id)),
        }
    }

    pub fn get_element_size(&self, element_id: u32) -> Result<Vec2, String> {
        match self.get_element(element_id) {
            Some(element) => Ok(element.get_size()),
            None => Err(format!("failed to get size because element with id {} was not found", element_id)),
        }
    }

    /// Get the position of the element as the center pixel (in world space)
    pub fn get_element_screen_position(&self, element_id: u32) -> Result<Vec2, String> {
        match self.get_element(element_id) {
            Some(element) => Ok(element.get_screen_position()),
            None => Err(format!("failed to get size because element with id {} was not found", element_id)),
        }
    }

    pub fn set_text(&mut self, text_element_id: u32, text: &String, asset_registry: &mut AssetRegistry) -> Result<(), String> {
        let window_size: Vec2 = self.size.clone();

        match self.get_mut_element(text_element_id) {
            Some(element) => {
                element.set_text(text, asset_registry, &window_size)
            },
            None => Err(format!("failed to set text because element with id {} was not found", text_element_id)),
        }
    }

    // map mouse position so that (0, 0) is the center
    pub fn map_mouse_position(&self, input: &Input) -> Vec2 {
        Vec2 {
            x: input.get_mouse_position_x() as f32 - self.size.x / 2.0,
            y: -(input.get_mouse_position_y() as f32 - self.size.y / 2.0),
        }
    }

    // If there is not an element currently being dragged, set it to the given element
    pub fn try_set_dragged_element(&mut self, element_id: u32) -> bool {
        let mut did_update = false;

        self.dragged_element_id = match self.dragged_element_id {
            Some(already_active) => { Some(already_active) },
            None => {
                did_update = true;
                Some(element_id)
            },
        };

        did_update
    }

    pub fn is_element_dragged(&mut self, element_id: u32) -> bool {
        match self.dragged_element_id {
            Some(dragged_element_id) => dragged_element_id == element_id,
            None => false,
        }
    }

    // TODO make these default values configurable
    pub fn default_font(&self, asset_registry: &mut AssetRegistry) -> Result<AssetId<Font>, String> {
        asset_registry.load_font(PlainBitmapBuilder::new()
            .with_font_file_path("./assets/fonts/roboto.ttf".to_string())
            .with_font_size(50.0)
        , None)
    }
    pub fn default_text_color(&self) -> (u8, u8, u8) { (239, 239, 239) }
    pub fn default_element_background_color(&self) -> (u8, u8, u8) { (56, 56, 56) }

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

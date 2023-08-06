use std::any::TypeId;

use glam::Vec2;

use crate::{asset_manager::{AssetManager, AssetId}, input::{Input, MouseButton}, graphics::{font::Font, ui::{element::{ui_element::UiElement, AnchorElementData}, Text, TextBuilder, shapes::{RectangleBuilder, Rectangle}, Position}}, log};

use super::{interface, element_list::{ElementList, OrderedElementsItem, self}, anchor_tree::{AnchorTree, AnchorElementIdentifier}};

const MIN_Z_INDEX: f32 = 1.0;
const MAX_Z_INDEX: f32 = 10_000.0;

pub struct ElementRegistry {
    text_elements: ElementList<Text>,
    rectangle_elements: ElementList<Rectangle>,

    /// list of ordered elements, ordered by z_index, so list starts with 
    /// the elements with the highest z_index
    ordered_elements: Vec<OrderedElementsItem>,

    window_size: Vec2,
    dragged_element_id: Option<u32>, // element that is currently being dragged. Will be set to None on left mouse button up

    anchor_tree: AnchorTree,
}

impl ElementRegistry {
    pub fn new(window_size: Vec2) -> Self {
        Self {
            text_elements: ElementList::<Text>::new(),
            rectangle_elements: ElementList::<Rectangle>::new(),
            ordered_elements: vec![],

            window_size,
            dragged_element_id: None,

            anchor_tree: AnchorTree::new(),
        }
    }

    pub fn update(&mut self, _asset_manager: &mut AssetManager, input: &Input) {
        if input.is_mouse_button_up(MouseButton::Left) {
            self.dragged_element_id = None;
        }
    }

    pub fn draw(&self, asset_manager: &mut AssetManager) {
        for ordered_item in self.ordered_elements.iter() {
            match self.get_ui_element_by_index(ordered_item.element_type, ordered_item.index) {
                Some(element) => {
                    element.draw(asset_manager);
                },
                None => {
                    log::engine_warn(format!("Failed to draw ElementRegistry item because we could not get it from ordered item {:?}", ordered_item));
                },
            }
        }
    }

    /// The prefered way of getting a ui_element
    fn get_ui_element_by_index(&self, type_id: TypeId, index: usize) -> Option<Box<&dyn UiElement>> {
        if type_id == TypeId::of::<Rectangle>() {
            match self.rectangle_elements.get_by_index(index) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }
        } else if type_id == TypeId::of::<Text>() {
            match self.text_elements.get_by_index(index) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }     
        } else {
            panic!("Unhandled element type")
        }
    }
    fn get_mut_ui_element_by_index(&mut self, type_id: TypeId, index: usize) -> Option<Box<&mut dyn UiElement>> {
        if type_id == TypeId::of::<Rectangle>() {
            match self.rectangle_elements.get_mut_by_index(index) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }
        } else if type_id == TypeId::of::<Text>() {
            match self.text_elements.get_mut_by_index(index) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }     
        } else {
            panic!("Unhandled element type")
        }
    }

    fn get_ui_element_by_type_and_id(&self, type_id: TypeId, id: u32) -> Option<Box<&dyn UiElement>> {
        if type_id == TypeId::of::<Rectangle>() {
            match self.rectangle_elements.get_by_id(id) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }
        } else if type_id == TypeId::of::<Text>() {
            match self.text_elements.get_by_id(id) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }  
        } else {
            panic!("Unhandled element type")
        }
    }
    fn get_mut_ui_element_by_type_and_id(&mut self, type_id: TypeId, id: u32) -> Option<Box<&mut dyn UiElement>> {
        if type_id == TypeId::of::<Rectangle>() {
            match self.rectangle_elements.get_mut_by_id(id) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }
        } else if type_id == TypeId::of::<Text>() {
            match self.text_elements.get_mut_by_id(id) {
                Some(el) => Some(Box::new(el)),
                None => None,
            }  
        } else {
            panic!("Unhandled element type")
        }
    }

    /// If the itemType and index are known, it's better to use get_ui_element_by_index. This is because
    /// this function looks up the itemType and index and then uses get_ui_element_by_index
    fn get_ui_element_by_id(&self, id: u32) -> Option<Box<&dyn UiElement>> {
        for ordered_element in self.ordered_elements.iter() {
            if ordered_element.item_id == id {
                return self.get_ui_element_by_index(ordered_element.element_type, ordered_element.index);
            }
        }

        return None
    }
    fn get_mut_ui_element_by_id(&mut self, id: u32) -> Option<Box<&mut dyn UiElement>> {
        for ordered_element in self.ordered_elements.iter() {
            if ordered_element.item_id == id {
                return self.get_mut_ui_element_by_index(ordered_element.element_type, ordered_element.index);
            }
        }

        return None
    }

    pub fn create_text(&mut self, text: String, font_id: Option<&AssetId<Font>>, text_builder: TextBuilder, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let font_id_to_use = match font_id {
            Some(id) => id.duplicate(),
            None => interface::default_font(asset_manager)?,
        };

        let text_element = Text::new(text, &font_id_to_use, text_builder, asset_manager, self)?;
        Ok(self.add_text(text_element))
    }

    pub fn add_text(&mut self, text_element: Text) -> u32 {
        let id = self.text_elements.add(text_element);

        self.update_ordered_elements();

        let position = self.text_elements.last().unwrap().world_data().position_type().clone();
        self.register_in_anchor_tree(TypeId::of::<Text>(), id, &position);
        
        id
    }

    pub fn create_rectangle(&mut self, builder: RectangleBuilder, asset_manager: &mut AssetManager) -> Result<u32, String> {
        let rectangle_element = Rectangle::new(builder, asset_manager, self)?;
        Ok(self.add_rectangle(rectangle_element))
    }

    pub fn add_rectangle(&mut self, rectangle_element: Rectangle) -> u32 {
        let id = self.rectangle_elements.add(rectangle_element);

        self.update_ordered_elements();

        let position = self.rectangle_elements.last().unwrap().world_data().position_type().clone();
        self.register_in_anchor_tree(TypeId::of::<Rectangle>(), id, &position);

        id
    }

    fn update_ordered_elements(&mut self) {
        let mut ordered_elements: Vec<OrderedElementsItem> = vec![];
        
        // Append the ordered elements of every element list
        ordered_elements.append(&mut self.text_elements.ordered_element_items());
        ordered_elements.append(&mut self.rectangle_elements.ordered_element_items());

        ordered_elements.sort_by(|a, b| a.z_index.total_cmp(&b.z_index));

        self.ordered_elements = ordered_elements;
    }

    pub fn register_in_anchor_tree(&mut self, type_id: TypeId, element_id: u32, position: &Position) {
        match position {
            Position::Fixed(_, _) => self.anchor_tree.add_fixed_anchor(type_id, element_id),
            Position::ScreenAnchor(_) => self.anchor_tree.add_screen_anchor(type_id, element_id),
            Position::ElementAnchor(_, anchor_id) => {
                for ordered_element in self.ordered_elements.iter() {
                    if ordered_element.item_id == *anchor_id {
                        self.anchor_tree.add_element_anchor(
                            ordered_element.element_type, 
                            ordered_element.item_id, 
                            type_id, 
                            element_id
                        );

                        return;
                    }
                }

                log::engine_warn(format!("Failed to register element in anchor tree because we could not found its anchor element by id {}", anchor_id));
            },
        }
    }

    pub fn handle_window_resize(&mut self, window_size: Vec2, asset_manager: &mut AssetManager) {
        self.window_size = window_size.clone();
        let view_uniform = to_view_uniform(self.window_size.x, self.window_size.y);

        // update view uniform of all ui elements
        for i in 0..self.ordered_elements.len() {
            let element_type = self.ordered_elements[i].element_type;
            let element_index = self.ordered_elements[i].index;

            match self.get_mut_ui_element_by_index(element_type, element_index) {
                Some(element) => {
                    element.handle_window_resize(&window_size);

                    let shader_id;

                    match asset_manager.get_material_by_id(element.material_id()) {
                        Some(material) => {
                            shader_id = material.shader_id.duplicate();
                        }
                        None => continue,
                    }

                    asset_manager.get_shader_by_id(&shader_id).unwrap().set_uniform("view", view_uniform);
                },
                None => {
                    log::engine_warn(format!("Failed to handle ElementRegistry window resize for element because we could not get it from ordered item {:?}"
                    , self.ordered_elements[i]));
                },
            }
        }
    }

    pub fn generate_element_id(&mut self) -> u32 {
        element_list::generate_id()
    }

    pub fn is_element_hovered(&self, element_id: u32, input: &Input) -> bool {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => {
                element.world_data().is_within(self.map_mouse_position(&input))
            }
            None => {
                log::engine_warn(format!("ElementRegistry.is_element_hovered for element id {} returned false because element was not found", element_id));
                false
            }
        }
    }

    pub fn is_element_clicked(&self, element_id: u32, input: &Input) -> bool {
        return input.is_mouse_button_down(MouseButton::Left) 
            && self.is_element_hovered(element_id, input)
    }

    pub fn get_element_scale(&self, element_id: u32) -> Result<Vec2, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.get_scale()),
            None => Err(format!("failed to get scale because element with id {} was not found", element_id)),
        }
    }

    pub fn set_element_scale(&mut self, element_id: u32, scale: Vec2) -> Result<(), String> {
        let window_size = self.size().clone();

        let anchor_element_data = self.get_anchor_element_data(element_id)?;

        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                element.set_scale(scale, window_size, anchor_element_data);
                self.update_anchor_tree(element_id);
                Ok(())
            },
            None => Err(format!("failed to set scale because element with id {} was not found", element_id)),
        }
    }

    pub fn update_anchor_tree(&mut self, element_id: u32) {
        let parent = self.anchor_tree.get_by_id(element_id).unwrap().identifier().clone();
        self.update_anchor_element_position(&parent);

        let children = self.anchor_tree.get_children(element_id);
        for child in children.iter() {
            self.update_anchor_element_position(&child);
        }
    }

    fn update_anchor_element_position(&mut self, anchor_identifier: &AnchorElementIdentifier) {
        let anchor_element_data = self.get_anchor_element_data(anchor_identifier.element_id).unwrap();

        if anchor_identifier.type_id == TypeId::of::<Rectangle>() {
            self.rectangle_elements
                .get_mut_by_id(anchor_identifier.element_id).unwrap()
                .recalculate_position(self.window_size.clone(), anchor_element_data);
        } else if anchor_identifier.type_id == TypeId::of::<Text>() {
            self.text_elements
                .get_mut_by_id(anchor_identifier.element_id).unwrap()
                .recalculate_position(self.window_size.clone(), anchor_element_data);
        } else {
            panic!("Unhandled element type")
        }
    }

    /// Get anchor element data of the anchor element of the given element
    fn get_anchor_element_data(&self, element_id: u32) -> Result<Option<AnchorElementData>, String> {
        match self.get_anchor_element_id(element_id)? {
            Some(anchor_element_id) => {
                Ok(Some(self.get_anchor_data(anchor_element_id).unwrap()))
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
        match self.get_ui_element_by_id(element_id) {
            Some(element) => {
                Ok(element.world_data().position_type().get_anchor_element_id())
            },
            None => Err(format!("failed to get anchor element id because element with id {} was not found", element_id)),
        }
    }

    /// get the base size of the element, not counting it's scale
    pub fn get_element_base_size(&self, element_id: u32) -> Result<Vec2, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.get_size()),
            None => Err(format!("failed to get size because element with id {} was not found", element_id)),
        }
    }

    /// get the base size of the element multiplied by its scale
    pub fn get_element_size(&self, element_id: u32) -> Result<Vec2, String> {        
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.get_size() * element.get_scale()),
            None => Err(format!("failed to get size because element with id {} was not found", element_id)),
        }
    }

    /// Get the position of the element as the center pixel (in world space)
    pub fn get_element_screen_position(&self, element_id: u32) -> Result<Vec2, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.get_screen_position()),
            None => Err(format!("failed to get size because element with id {} was not found", element_id)),
        }
    }

    pub fn set_text(&mut self, text_element_id: u32, text: &String, asset_manager: &mut AssetManager) -> Result<(), String> {
        let window_size: Vec2 = self.window_size.clone();
        let anchor_data = self.get_anchor_element_data(text_element_id)?;

        match self.text_elements.get_mut_by_id(text_element_id) {
            Some(text_element) => {
                text_element.set_text(text, window_size, anchor_data, asset_manager)?;
                self.update_anchor_tree(text_element_id);
                Ok(())
            },
            None => Err(format!("failed to set text because element with id {} was not found", text_element_id)),
        }
    }

    // map mouse position so that (0, 0) is the center
    pub fn map_mouse_position(&self, input: &Input) -> Vec2 {
        Vec2 {
            x: input.get_mouse_position_x() as f32 - self.window_size.x / 2.0,
            y: -(input.get_mouse_position_y() as f32 - self.window_size.y / 2.0),
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

    pub fn print_anchor_tree(&self) {
        self.anchor_tree.print();
    }

    pub fn size(&self) -> &Vec2 { &self.window_size }
    pub fn width(&self) -> f32 { self.window_size.x }
    pub fn height(&self) -> f32 { self.window_size.y }
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

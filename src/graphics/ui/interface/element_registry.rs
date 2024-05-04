use std::any::TypeId;

use glam::Vec2;

use crate::{asset_manager::AssetManager, graphics::{font::Font, shader::{CustomShaderValues, UniformValue}, ui::{bounds_2d::Bounds2d, element::{ui_element::UiElement, world_element_data::WorldElementData, AnchorElementData, InputEvent}, shapes::{Rectangle, RectangleBuilder}, Position, Text, TextBuilder, UiElementId}, Color}, input::{Input, ButtonAction, MouseButton}, log, ResourceId};

use super::{interface, element_list::{ElementList, OrderedElementsItem, self}, anchor_tree::{AnchorTree, AnchoredElement, AnchorElementIdentifier}};

pub const MIN_Z_INDEX: f32 = 1.0;
pub const MAX_Z_INDEX: f32 = 10_000.0;

pub struct ElementRegistry {
    text_elements: ElementList<Text>,
    rectangle_elements: ElementList<Rectangle>,

    /// list of ordered elements, ordered by z_index, so that the list starts with 
    /// the elements with the lowest z_index and ends with the element with the highest 
    /// z_index. This means the last element in the list gets drawn on top of every other
    /// element.
    ordered_elements: Vec<OrderedElementsItem>,

    window_size: Vec2,
    pixel_density: f32,

    anchor_tree: AnchorTree,
}

impl ElementRegistry {
    pub fn new(window_size: Vec2, pixel_density: f32) -> Self {
        Self {
            text_elements: ElementList::<Text>::new(),
            rectangle_elements: ElementList::<Rectangle>::new(),
            ordered_elements: vec![],

            window_size,
            pixel_density,

            anchor_tree: AnchorTree::new(),
        }
    }

    pub fn update(&mut self, _asset_manager: &mut dyn AssetManager, input: &Input) {
        self.handle_inputs_events(input);
    }

    fn handle_inputs_events(&mut self, input: &Input) {
        self.text_elements.reset_event_handlers(input);
        self.rectangle_elements.reset_event_handlers(input);

        self.handle_input_event(InputEvent::Hover, input);

        if input.mouse.has_scroll() {
            self.handle_input_event(InputEvent::Scroll, input);
        }

        if input.mouse.is_button_down(MouseButton::Left) {
            self.handle_input_event(InputEvent::MouseLeftDown, input);

            if !input.mouse.is_button_up(MouseButton::Left) {
                self.handle_input_event(InputEvent::MouseLeftDrag, input);
            }
        } 
        if input.mouse.is_button_up(MouseButton::Left) {
            self.handle_input_event(InputEvent::MouseLeftUp, input);
        }
    }

    fn handle_input_event(&mut self, event: InputEvent, input: &Input) {
        let mouse_position = self.map_mouse_position(input);

        for i in (0..self.ordered_elements.len()).rev() {
            let element_type = self.ordered_elements[i].element_type;
            let element_index = self.ordered_elements[i].index;

            match self.get_mut_ui_element_by_index(element_type, element_index) {
                Some(element) => {                    
                    if element.world_data().show && element.world_data().is_within(mouse_position) && element.mut_world_data().event_handlers.register_event(event, input) {
                        break;
                    }
                },
                None => {
                    log::engine_warn(format!("Failed to handle ElementRegistry input for element because we could not get it from ordered item {:?}"
                    , self.ordered_elements[i]));
                },
            }
        }
    }
    
    pub fn set_pixel_density(&mut self, pixel_density: f32) {
        self.pixel_density = pixel_density;
    }

    pub fn draw(&self, asset_manager: &mut dyn AssetManager) {
        for ordered_item in self.ordered_elements.iter() {
            match self.get_ui_element_by_index(ordered_item.element_type, ordered_item.index) {
                Some(element) => {
                    element.draw(asset_manager, &self.window_size, self.pixel_density);
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

    fn get_ui_element_by_type_and_id(&self, type_id: TypeId, id: &ResourceId<UiElementId>) -> Option<Box<&dyn UiElement>> {
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
    fn get_mut_ui_element_by_type_and_id(&mut self, type_id: TypeId, id: &ResourceId<UiElementId>) -> Option<Box<&mut dyn UiElement>> {
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
    pub fn get_ui_element_by_id(&self, id: &ResourceId<UiElementId>) -> Option<Box<&dyn UiElement>> {
        for ordered_element in self.ordered_elements.iter() {
            if ordered_element.item_id.equals(id) {
                return self.get_ui_element_by_index(ordered_element.element_type, ordered_element.index);
            }
        }

        return None
    }
    fn get_mut_ui_element_by_id(&mut self, id: &ResourceId<UiElementId>) -> Option<Box<&mut dyn UiElement>> {
        for ordered_element in self.ordered_elements.iter() {
            if ordered_element.item_id.equals(id) {
                return self.get_mut_ui_element_by_index(ordered_element.element_type, ordered_element.index);
            }
        }

        return None
    }

    pub fn create_text(&mut self, text: impl Into<String>, font_id: Option<&ResourceId<Box<dyn Font>>>, text_builder: &TextBuilder, asset_manager: &mut dyn AssetManager) -> Result<ResourceId<UiElementId>, String> {
        let font_id_to_use = match font_id {
            Some(id) => id.duplicate(),
            None => interface::default_font(asset_manager)?,
        };

        let text_element = text_builder.build(text, &font_id_to_use, asset_manager, self)?;
        self.on_create_element(&text_element, asset_manager);

        Ok(self.add_text(text_element))
    }

    fn on_create_element(&self, element: &impl UiElement, asset_manager: &mut dyn AssetManager) {
        // TODO We only need to set this view uniform when the shader has been newly made.
        asset_manager.get_material_shader(element.material_id()).unwrap().set_uniform(
            "view", 
            &to_view_uniform(self.window_size.x, self.window_size.y)
        );
    }

    pub fn add_text(&mut self, text_element: Text) -> ResourceId<UiElementId> {
        let id = self.text_elements.add(text_element);

        self.update_ordered_elements();

        let position = self.text_elements.last().unwrap().world_data().position_type().clone();
        self.register_in_anchor_tree(TypeId::of::<Text>(), id.clone(), &position);
        
        id
    }

    pub fn create_rectangle(&mut self, builder: &RectangleBuilder, asset_manager: &mut dyn AssetManager) -> Result<ResourceId<UiElementId>, String> {
        let rectangle_element = builder.build(asset_manager, self)?;
        self.on_create_element(&rectangle_element, asset_manager);

        Ok(self.add_rectangle(rectangle_element))
    }

    pub fn add_rectangle(&mut self, rectangle_element: Rectangle) -> ResourceId<UiElementId> {
        let id = self.rectangle_elements.add(rectangle_element);

        self.update_ordered_elements();

        let position = self.rectangle_elements.last().unwrap().world_data().position_type().clone();
        self.register_in_anchor_tree(TypeId::of::<Rectangle>(), id, &position);

        id
    }

    pub fn remove_element(&mut self, element_id: &ResourceId<UiElementId>) -> Result<(), String> {
        if !self.remove_element_from_element_list(element_id) {
            return Err(format!("Element not found in element lists"));
        }

        match self.anchor_tree.remove_element_by_id(element_id) {
            Some(mut removed_element) => {
                for child in removed_element.take_children() {
                    self.remove_anchored_element(child)?;
                }
            },
            None => {
                return Err(format!("Element not found in element lists"));
            }, 
        };

        self.update_ordered_elements();

        Ok(())
    }

    fn remove_element_from_element_list(&mut self, element_id: &ResourceId<UiElementId>) -> bool {
        return self.text_elements.remove(element_id) 
            || self.rectangle_elements.remove(element_id);
    }

    /// Recursively remove the element of the anchored element from the element lists
    fn remove_anchored_element(&mut self, mut anchored_element: AnchoredElement) -> Result<(), String> {
        if !self.remove_element_from_element_list(&anchored_element.identifier().element_id) {
            return Err(format!("Element not found in element lists"));
        }

        for child in anchored_element.take_children() {
            self.remove_anchored_element(child)?;
        }

        Ok(())
    }

    fn update_ordered_elements(&mut self) {
        let mut ordered_elements: Vec<OrderedElementsItem> = vec![];
        
        // Append the ordered elements of every element list
        ordered_elements.append(&mut self.text_elements.ordered_element_items());
        ordered_elements.append(&mut self.rectangle_elements.ordered_element_items());

        ordered_elements.sort_by(|a, b| a.z_index.total_cmp(&b.z_index));

        self.ordered_elements = ordered_elements;
    }

    fn register_in_anchor_tree(&mut self, type_id: TypeId, element_id: ResourceId<UiElementId>, position: &Position) {
        match position {
            Position::Fixed(_, _) => self.anchor_tree.add_fixed_anchor(type_id, element_id),
            Position::ScreenAnchor(_) => self.anchor_tree.add_screen_anchor(type_id, element_id),
            Position::ElementAnchor(_, anchor_id) => {
                for ordered_element in self.ordered_elements.iter() {
                    if ordered_element.item_id.equals(anchor_id) {
                        match self.anchor_tree.add_element_anchor(
                            ordered_element.element_type, 
                            &ordered_element.item_id, 
                            type_id, 
                            element_id
                        ) {
                            Ok(_) => {},
                            Err(err) => {
                                log::engine_warn(format!("Failed to register element in anchor tree as element anchor: {:?}", err));
                            },
                        }

                        return
                    }
                }

                log::engine_warn(format!("Failed to register element in anchor tree because we could not found its anchor element by id {:?}", anchor_id));
            },
        }
    }

    pub fn handle_window_resize(&mut self, window_size: Vec2, asset_manager: &mut dyn AssetManager) {
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

                    asset_manager.get_shader_by_id(&shader_id).unwrap().set_uniform("view", &view_uniform);
                },
                None => {
                    log::engine_warn(format!("Failed to handle ElementRegistry window resize for element because we could not get it from ordered item {:?}"
                    , self.ordered_elements[i]));
                },
            }
        }

        // update positions of screen-anchored elements
        for screen_tree_root_element in self.anchor_tree.root_screen_tree_elements() {
            match self.get_mut_ui_element_by_id(&screen_tree_root_element.element_id) {
                Some(element) => {
                    // since we only deal with root elements here, we can pass None as anchor_element_data since root elements
                    // are never anchored to another element
                    element.mut_world_data().calculate_position(window_size, None);

                    // update positions of the children of our current root element
                    match self.update_anchor_tree(&screen_tree_root_element.element_id) {
                        Ok(_) => {},
                        Err(err) => {
                            log::engine_err(
                                format!("failed to handle window resize for screen anchored root element with id {}: {}", screen_tree_root_element.element_id.id(), err)
                            );
                        },
                    }
                },
                None => log::engine_err(
                    format!("failed to handle window resize for screen anchored root element with id {}", screen_tree_root_element.element_id.id())
                ),
            }
        }
    }

    pub fn generate_element_id(&mut self) -> u32 {
        element_list::generate_id()
    }

    pub fn is_element_hovered(&self, element_id: &ResourceId<UiElementId>) -> bool {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => {
                element.world_data().event_handlers.hover_handler.did_handle()
            }
            None => {
                log::engine_warn(format!("ElementRegistry.is_element_hovered for element id {} returned false because element was not found", element_id.id()));
                false
            }
        }
    }

    pub fn is_element_clicked(&self, element_id: &ResourceId<UiElementId>, mouse_button: MouseButton, input_action: &ButtonAction) -> bool {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => {
                element.world_data().event_handlers.did_handle_mouse_event(&mouse_button, input_action)
            },
            None => {
                log::engine_warn(format!("ElementRegistry.is_element_clicked for element id {} returned false because element was not found", element_id.id()));
                false
            },
        }
    }

    pub fn is_element_dragged(&self, element_id: &ResourceId<UiElementId>, mouse_button: MouseButton) -> bool {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => {
                match mouse_button {
                    MouseButton::Left => {
                        return element.world_data().event_handlers.mouse_left_drag_handler.is_handling();
                    },
                    _ => {
                        log::engine_warn(format!("ElementRegistry.is_element_dragged for element id {} returns false because mouse button {:?} is not handled", element_id.id(), mouse_button));
                        false
                    },
                }
            },
            None => {
                log::engine_warn(format!("ElementRegistry.is_element_dragged for element id {} returned false because element was not found", element_id.id()));
                false
            },
        }
    }

    pub fn get_element_scale(&self, element_id: &ResourceId<UiElementId>) -> Result<Vec2, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.world_data().scale()),
            None => Err(format!("failed to get scale because element with id {:?} was not found", element_id)),
        }
    }

    pub fn get_mut_element_custom_shader_values(&mut self, element_id: &ResourceId<UiElementId>) -> Result<&mut CustomShaderValues, String> {
        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                Ok(element.mut_custom_shader_values())
            },
            None => Err(format!("element with id {:?} was not found", element_id)),
        }
    }

    pub fn set_element_color(&mut self, element_id: &ResourceId<UiElementId>, color: Color) -> Result<(), String> {
        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                Ok(element.set_color(color))
            },
            None => Err(format!("failed to set element color because element with id {:?} was not found", element_id)),
        }
    }

    pub fn set_element_z_index(&mut self, element_id: &ResourceId<UiElementId>, z_index: f32) -> Result<(), String> {
        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                element.mut_world_data().z_index = z_index;
                self.update_ordered_elements();
                Ok(())
            },
            None => Err(format!("failed to set element color because element with id {:?} was not found", element_id)),
        }
    }

    pub fn set_element_draw_bounds(&mut self, element_id: &ResourceId<UiElementId>, draw_bounds: Bounds2d) -> Result<(), String> {
        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                Ok(element.mut_world_data().draw_bounds = draw_bounds)
            },
            None => Err(format!("failed to set element color because element with id {:?} was not found", element_id)),
        }
    }

    pub fn set_element_position_transform(&mut self, element_id: &ResourceId<UiElementId>, position_transform: Vec2) -> Result<(), String> {
        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                element.mut_world_data().position_transform = position_transform;
                return self.update_anchor_tree(element_id);
            },
            None => Err(format!("failed to set element position_transform because element with id {:?} was not found", element_id)),
        }
    }

    fn reposition_anchor_tree_element(&mut self, element_id: &ResourceId<UiElementId>, position: Position) {
        let removed_element = self.anchor_tree.remove_element_by_id(element_id);
        self.register_in_anchor_tree(self.get_type_id(element_id).unwrap(), element_id.clone(), &position);

        if let Some(mut anchored_element) = removed_element {
            self.anchor_tree.add_children(element_id, anchored_element.take_children()).unwrap();
        }
    }

    pub fn set_element_position(&mut self, element_id: &ResourceId<UiElementId>, position: Position) -> Result<(), String> {
        self.reposition_anchor_tree_element(element_id, position);

        let window_size = self.window_size.clone();
        let anchor_element_data: Option<AnchorElementData> = Some(self.get_anchor_data(element_id)?);
        
        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                element.mut_world_data().set_position(position, window_size, anchor_element_data);
                return self.update_anchor_tree(element_id);
            },
            None => Err(format!("failed to set element position because element with id {:?} was not found", element_id)),
        }
    }

    fn get_type_id(&self, element_id: &ResourceId<UiElementId>) -> Option<TypeId> {
        for ordered_element in self.ordered_elements.iter() {
            if ordered_element.item_id.equals(element_id) {
                return Some(ordered_element.element_type);
            }
        }

        None
    }

    pub fn set_element_scale(&mut self, element_id: &ResourceId<UiElementId>, scale: Vec2) -> Result<(), String> {
        let window_size = self.size().clone();

        let anchor_element_data = self.get_anchor_element_data(element_id)?;

        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                element.mut_world_data().set_scale(scale, window_size, anchor_element_data);
                return self.update_anchor_tree(element_id);
            },
            None => Err(format!("failed to set scale because element with id {:?} was not found", element_id)),
        }
    }

    fn update_anchor_tree(&mut self, element_id: &ResourceId<UiElementId>) -> Result<(), String> {
        let parent = self.anchor_tree.get_by_id(element_id).unwrap().identifier().clone();
        self.update_anchor_element_position(&parent)?;

        let children = self.anchor_tree.get_children(element_id);
        for child in children.iter() {
            self.update_anchor_element_position(&child)?;
        }

        Ok(())
    }

    fn update_anchor_element_position(&mut self, anchor_identifier: &AnchorElementIdentifier) -> Result<(), String> {
        let anchor_element_data = self.get_anchor_element_data(&anchor_identifier.element_id)?;

        if anchor_identifier.type_id == TypeId::of::<Rectangle>() {
            self.rectangle_elements
                .get_mut_by_id(&anchor_identifier.element_id).unwrap()
                .mut_world_data()
                .calculate_position(self.window_size.clone(), anchor_element_data);
        } else if anchor_identifier.type_id == TypeId::of::<Text>() {
            self.text_elements
                .get_mut_by_id(&anchor_identifier.element_id).unwrap()
                .mut_world_data()
                .calculate_position(self.window_size.clone(), anchor_element_data);
        } else {
            return Err(format!("Unhnadled type {:?}", anchor_identifier.type_id));
        }

        Ok(())
    }

    /// Get anchor element data of the anchor element of the given element
    pub fn get_anchor_element_data(&self, element_id: &ResourceId<UiElementId>) -> Result<Option<AnchorElementData>, String> {
        match self.get_anchor_element_id(element_id)? {
            Some(anchor_element_id) => {
                Ok(Some(self.get_anchor_data(&anchor_element_id).unwrap()))
            },
            None => Ok(None),
        }
    }

    /// Get the anchor element data of the given element
    pub fn get_anchor_data(&self, anchor_element_id: &ResourceId<UiElementId>) -> Result<AnchorElementData, String> {
        Ok(AnchorElementData{
            id: anchor_element_id.clone(),
            size: self.get_element_size(anchor_element_id)?,
            coordinates: self.get_element_screen_position(anchor_element_id)?,
        })
    }

    pub fn get_anchor_element_id(&self, element_id: &ResourceId<UiElementId>) -> Result<Option<ResourceId<UiElementId>>, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => {
                Ok(element.world_data().position_type().get_anchor_element_id())
            },
            None => Err(format!("failed to get anchor element id because element with id {} was not found", element_id.id())),
        }
    }

    /// get the base size of the element, not counting it's scale
    pub fn get_element_base_size(&self, element_id: &ResourceId<UiElementId>) -> Result<Vec2, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.world_data().size()),
            None => Err(format!("failed to get size because element with id {:?} was not found", element_id)),
        }
    }

    /// get the base size of the element multiplied by its scale
    pub fn get_element_size(&self, element_id: &ResourceId<UiElementId>) -> Result<Vec2, String> {        
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.world_data().size() * element.world_data().scale()),
            None => Err(format!("failed to get size because element with id {:?} was not found", element_id)),
        }
    }

    /// Get the position of the element as the center pixel (in world space)
    pub fn get_element_screen_position(&self, element_id: &ResourceId<UiElementId>) -> Result<Vec2, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.world_data().position()),
            None => Err(format!("failed to get size because element with id {:?} was not found", element_id)),
        }
    }

    pub fn get_element_position_transform(&self, element_id: &ResourceId<UiElementId>) -> Result<Vec2, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => Ok(element.world_data().position_transform),
            None => Err(format!("failed to get position transform because element with id {:?} was not found", element_id.id())),
        }
    }

    pub fn set_text(&mut self, text_element_id: &ResourceId<UiElementId>, text: &String, asset_manager: &mut dyn AssetManager) -> Result<(), String> {
        let window_size: Vec2 = self.window_size.clone();
        let anchor_data = self.get_anchor_element_data(text_element_id)?;

        match self.text_elements.get_mut_by_id(text_element_id) {
            Some(text_element) => {
                text_element.set_text(text, window_size, anchor_data, asset_manager)?;
                return self.update_anchor_tree(text_element_id);
            },
            None => Err(format!("failed to set text because text element with id {:?} was not found", text_element_id)),
        }
    }

    pub fn set_rectangle_width(&mut self, rectangle_id: &ResourceId<UiElementId>, new_width: f32) -> Result<(), String> {
        let window_size: Vec2 = self.window_size.clone();
        let anchor_data = self.get_anchor_element_data(rectangle_id)?;

        match self.rectangle_elements.get_mut_by_id(rectangle_id) {
            Some(rectangle) => {
                rectangle.set_width(new_width, window_size, anchor_data);
                return self.update_anchor_tree(rectangle_id);
            },
            None => Err(format!("failed to set rectangle width because rectanglei with id {:?} was not found", rectangle_id)),
        }
    }
    pub fn set_rectangle_height(&mut self, rectangle_id: &ResourceId<UiElementId>, new_height: f32) -> Result<(), String> {
        let window_size: Vec2 = self.window_size.clone();
        let anchor_data = self.get_anchor_element_data(rectangle_id)?;

        match self.rectangle_elements.get_mut_by_id(rectangle_id) {
            Some(rectangle) => {
                rectangle.set_height(new_height, window_size, anchor_data);
                return self.update_anchor_tree(rectangle_id);
            },
            None => Err(format!("failed to set rectangle height because rectanglei with id {:?} was not found", rectangle_id)),
        }
    }
    pub fn set_rectangle_size(&mut self, rectangle_id: &ResourceId<UiElementId>, new_size: Vec2) -> Result<(), String> {
        let window_size: Vec2 = self.window_size.clone();
        let anchor_data = self.get_anchor_element_data(rectangle_id)?;

        match self.rectangle_elements.get_mut_by_id(rectangle_id) {
            Some(rectangle) => {
                rectangle.set_size(new_size, window_size, anchor_data);
                return self.update_anchor_tree(rectangle_id);
            },
            None => Err(format!("failed to set rectangle size because rectanglei with id {:?} was not found", rectangle_id)),
        }
    }
    pub fn get_rectangle_border(&mut self, rectangle_id: &ResourceId<UiElementId>) -> &super::super::shapes::RectangleBorder {
        self.rectangle_elements.get_by_id(rectangle_id).unwrap().get_border()
    }
    pub fn get_mut_rectangle_border(&mut self, rectangle_id: &ResourceId<UiElementId>) -> &mut super::super::shapes::RectangleBorder {
        self.rectangle_elements.get_mut_by_id(rectangle_id).unwrap().get_mut_border()
    }
    pub fn set_rectangle_texture_padding(&mut self, rectangle_id: &ResourceId<UiElementId>, texture_padding: f32) -> Result<(), String> {
        match self.rectangle_elements.get_mut_by_id(rectangle_id) {
            Some(rectangle) => { Ok(rectangle.set_texture_padding(texture_padding)) },
            None => Err(format!("failed to set rectangle texture padding because rectanglei with id {:?} was not found", rectangle_id)),
        }
    }

    pub fn set_element_visibility(&mut self, element_id: &ResourceId<UiElementId>, visible: bool) -> Result<(), String> {
        match self.get_mut_ui_element_by_id(element_id) {
            Some(element) => {
                Ok(element.mut_world_data().show = visible)
            },
            None => Err(format!("failed to set element visibility because element with id {:?} was not found", element_id)),
        }
    }
    
    pub fn element_world_data(&self, element_id: &ResourceId<UiElementId>) -> Result<&WorldElementData, String> {
        match self.get_ui_element_by_id(element_id) {
            Some(element) => {
                Ok(element.world_data())
            },
            None => Err(format!("failed to check if element is shown because element with id {:?} was not found", element_id)),
        }
    }

    /// Map mouse position to screen coordinates (in pixels) so that (0, 0) is the center
    pub fn map_mouse_position(&self, input: &Input) -> Vec2 {
        Vec2 {
            x: input.mouse.get_position_x() as f32 - self.window_size.x / 2.0,
            y: -(input.mouse.get_position_y() as f32 - self.window_size.y / 2.0),
        }
    }

    /// For debugging
    pub fn print_anchor_tree(&self) {
        self.anchor_tree.print();
    }

    pub fn size(&self) -> &Vec2 { &self.window_size }
    pub fn width(&self) -> f32 { self.window_size.x }
    pub fn height(&self) -> f32 { self.window_size.y }
}

fn to_view_uniform(window_width: f32, window_height: f32) -> UniformValue {
    UniformValue::from(
        (2.0 / window_width, 2.0 / window_height) // 0.5 because the world coordinates for the UI range from (-size / 2) to (size / 2)
    )
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

use std::any::TypeId;

use crate::graphics::ui::element::ui_element::UiElement;

static mut CURRENT_ID: u32 = 0;
pub fn generate_id() -> u32 {
    unsafe {   
        CURRENT_ID += 1;
        CURRENT_ID
    }
}

struct ElementEntry<T: UiElement> {
    element: T,
    id: u32,
}

#[derive(Debug)]
pub struct OrderedElementsItem {
    pub element_type: TypeId,
    pub index: usize,
    pub z_index: f32,
    pub item_id: u32,
}

pub struct ElementList<T: UiElement> {
    elements: Vec<ElementEntry<T>>,
}

impl<T: UiElement + 'static> ElementList<T> {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn add(&mut self, element: T) -> u32 {
        let id = generate_id();
        self.elements.push(ElementEntry { element, id, });
        id
    }

    pub fn get_by_id(&self, id: u32) -> Option<&T> {
        for entry in self.elements.iter() {
            if entry.id == id {
                return Some(&entry.element);
            }
        }

        None
    }

    pub fn get_mut_by_id(&mut self, id: u32) -> Option<&mut T> {
        for entry in self.elements.iter_mut() {
            if entry.id == id {
                return Some(&mut entry.element);
            }
        }

        None
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        if index >= self.elements.len() {
            return None;
        }

        Some(&self.elements[index].element)
    }

    pub fn get_mut_by_index(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.elements.len() {
            return None;
        }

        Some(&mut self.elements[index].element)
    }

    pub fn ordered_element_items(&self) -> Vec<OrderedElementsItem> {
        let mut ordered_elements: Vec<OrderedElementsItem> = vec![];

        for i in 0..self.elements.len() {
            ordered_elements.push(OrderedElementsItem {
                element_type: TypeId::of::<T>(),
                index: i,
                z_index: self.elements[i].element.world_data().z_index(),
                item_id: self.elements[i].id,
            })
        }
    
        ordered_elements
    }
}

use std::any::TypeId;

/// An element that is anchored to a previous element in the tree, and has 0 or more elements anchored to it
pub struct AnchoredElement {
    type_id: TypeId,
    element_id: u32,
    anchored_elements: Vec<AnchoredElement>,
}

impl AnchoredElement {
    fn new(type_id: TypeId, element_id: u32) -> Self {
        Self { type_id, element_id, anchored_elements: vec![] }
    }

    /// Print itself to the standard output, for debugging purpouses
    pub fn print(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        println!("{}{}", tabs, self.element_id);
        for element in &self.anchored_elements {
            element.print(depth + 1);
        }
    }

    /// Returns none if no element was found
    pub fn get(&self, type_id: TypeId, element_id: u32) -> Option<&Self> {
        if self.type_id == type_id && self.element_id == element_id {
            return Some(self);
        }

        for element in self.anchored_elements.iter() {
            if let Some(matching_element) = element.get(type_id, element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    /// Returns none if no element was found
    pub fn get_mut(&mut self, type_id: TypeId, element_id: u32) -> Option<&mut Self> {
        if self.type_id == type_id && self.element_id == element_id {
            return Some(self);
        }

        for element in self.anchored_elements.iter_mut() {
            if let Some(matching_element) = element.get_mut(type_id, element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    /// Returns none if no element was found
    pub fn get_by_id(&self, element_id: u32) -> Option<&Self> {
        if self.element_id == element_id {
            return Some(self);
        }

        for element in self.anchored_elements.iter() {
            if let Some(matching_element) = element.get_by_id(element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    /// Returns none if no element was found
    pub fn get_mut_by_id(&mut self, element_id: u32) -> Option<&mut Self> {
        if self.element_id == element_id {
            return Some(self);
        }

        for element in self.anchored_elements.iter_mut() {
            if let Some(matching_element) = element.get_mut_by_id(element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    pub fn push(&mut self, type_id: TypeId, element_id: u32) {
        self.anchored_elements.push(Self::new(type_id, element_id));
    }

    pub fn anchored_elements(&self) -> &Vec<AnchoredElement> { &self.anchored_elements }
    pub fn type_id(&self) -> TypeId { self.type_id }
    pub fn element_id(&self) -> u32 { self.element_id }
}

pub struct AnchorTree {
    /// AnchoredElements of which the root is the screen
    screen_tree: Vec<AnchoredElement>,

    /// AnchoredElements of which the root is a fixed position
    fixed_trees: Vec<AnchoredElement>,
}

impl AnchorTree {
    pub fn new() -> Self {
        Self { 
            screen_tree: vec![],
            fixed_trees: vec![],
        }
    }

    /// Print itself to the standard output, for debugging purpouses
    pub fn print(&self) {
        println!("screen_tree:");
        for screen_element in self.screen_tree.iter() {
            screen_element.print(1);
        }

        println!("fixed_trees:");
        for fixed_element in self.fixed_trees.iter() {
            fixed_element.print(1);
        }
    }

    /// Add anchor that is anchored to a fixed position on the screen
    pub fn add_screen_anchor(&mut self, type_id: TypeId, element_id: u32) {
        self.screen_tree.push(AnchoredElement::new(type_id, element_id));
    }

    /// Add anchor that is anchored to a fixed position on the screen
    pub fn add_fixed_anchor(&mut self, type_id: TypeId, element_id: u32) {
        self.fixed_trees.push(AnchoredElement::new(type_id, element_id));
    }

    /// Add anchor that is anchored to another element
    pub fn add_element_anchor(&mut self, anchor_type_id: TypeId, anchor_element_id: u32, type_id: TypeId, element_id: u32) {
        match self.get_mut(anchor_type_id, anchor_element_id) {
            Some(anchor) => anchor.push(type_id, element_id),
            None => todo!(),
        }
    }

    pub fn get(&self, type_id: TypeId, element_id: u32) -> Option<&AnchoredElement> {
        for entry in self.screen_tree.iter() {
            match entry.get(type_id, element_id) {
                Some(element) => return Some(&element),
                None => (),
            }
        }

        None
    }

    pub fn get_mut(&mut self, type_id: TypeId, element_id: u32) -> Option<&mut AnchoredElement> {
        for entry in self.screen_tree.iter_mut() {
            match entry.get_mut(type_id, element_id) {
                Some(element) => return Some(element),
                None => (),
            }
        }

        None
    }

    pub fn get_by_id(&self, element_id: u32) -> Option<&AnchoredElement> {
        for entry in self.screen_tree.iter() {
            match entry.get_by_id(element_id) {
                Some(element) => return Some(&element),
                None => (),
            }
        }

        None
    }

    pub fn get_mut_by_id(&mut self, element_id: u32) -> Option<&mut AnchoredElement> {
        for entry in self.screen_tree.iter_mut() {
            match entry.get_mut_by_id(element_id) {
                Some(element) => return Some(element),
                None => (),
            }
        }

        None
    }
}

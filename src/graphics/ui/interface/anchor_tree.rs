use std::any::TypeId;

use crate::{ResourceId, graphics::ui::UiElementId};

#[derive(Debug, Clone)]
pub struct AnchorElementIdentifier {
    pub type_id: TypeId,
    pub element_id: ResourceId<UiElementId>,
}

/// An element that is anchored to a previous element in the tree, and has 0 or more elements anchored to it
pub struct AnchoredElement {
    identifier: AnchorElementIdentifier,
    anchored_elements: Vec<AnchoredElement>, // children
}

impl AnchoredElement {
    fn new(type_id: TypeId, element_id: ResourceId<UiElementId>) -> Self {
        Self { identifier: AnchorElementIdentifier { type_id, element_id }, anchored_elements: vec![] }
    }

    /// Print itself to the standard output, for debugging purpouses
    pub fn print(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        println!("{}{}", tabs, self.identifier.element_id.id());
        for element in &self.anchored_elements {
            element.print(depth + 1);
        }
    }

    /// Returns `None` if no element was found
    pub fn get(&self, type_id: TypeId, element_id: &ResourceId<UiElementId>) -> Option<&Self> {
        if self.identifier.type_id == type_id && self.identifier.element_id.equals(element_id) {
            return Some(self);
        }

        for element in self.anchored_elements.iter() {
            if let Some(matching_element) = element.get(type_id, element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    /// Returns `None` if no element was found
    pub fn get_mut(&mut self, type_id: TypeId, element_id: &ResourceId<UiElementId>) -> Option<&mut Self> {
        if self.identifier.type_id == type_id && self.identifier.element_id.equals(element_id) {
            return Some(self);
        }

        for element in self.anchored_elements.iter_mut() {
            if let Some(matching_element) = element.get_mut(type_id, element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    /// Returns `None` if no element was found
    pub fn get_by_id(&self, element_id: &ResourceId<UiElementId>) -> Option<&Self> {
        if self.identifier.element_id.equals(element_id) {
            return Some(self);
        }

        for element in self.anchored_elements.iter() {
            if let Some(matching_element) = element.get_by_id(element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    /// Returns `None` if no element was found
    pub fn get_mut_by_id(&mut self, element_id: &ResourceId<UiElementId>) -> Option<&mut Self> {
        if self.identifier.element_id.equals(element_id) {
            return Some(self);
        }

        for element in self.anchored_elements.iter_mut() {
            if let Some(matching_element) = element.get_mut_by_id(element_id) {
                return Some(matching_element);
            }
        }

        None
    }

    /// Returns `None` if the child element is not found
    pub fn get_parent(&self, child_id: &ResourceId<UiElementId>) -> Option<&Self> {
        for element in self.anchored_elements.iter() {
            if element.identifier.element_id.equals(child_id) {
                return Some(&self);
            }

            if let Some(parent) = element.get_parent(child_id) {
                return Some(parent);
            }
        }

        None
    }

    /// Returns `None` if the child element is not found
    pub fn get_parent_id(&mut self, child_id: &ResourceId<UiElementId>) -> Option<ResourceId<UiElementId>> {
        for i in 0..self.anchored_elements.len() {
            if self.anchored_elements[i].identifier.element_id.equals(child_id) {
                return Some(self.anchored_elements[i].identifier.element_id.clone());
            }

            if let Some(parent) = self.anchored_elements[i].get_parent_id(child_id) {
                return Some(parent);
            }
        }

        None
    }

    /// Recursively tries to find the element with the given element_id and remove it.
    /// 
    /// Returns wether the element was removed from the children:
    /// * `Some` if the element was found, and thus removed
    /// * `None` if the element was not found, thus is not a child
    pub fn remove_child_by_id(&mut self, element_id: &ResourceId<UiElementId>) -> Option<AnchoredElement> {
        for i in 0..self.anchored_elements.len() {
            if self.anchored_elements[i].identifier.element_id.equals(element_id) {
                return Some(self.anchored_elements.remove(i));
            } else {
                let mut remove_result = self.anchored_elements[i].remove_child_by_id(element_id);
                if remove_result.is_some() {
                    return remove_result.take();
                }
            }
        }

        return None;
    }

    pub fn push(&mut self, type_id: TypeId, element_id: ResourceId<UiElementId>) {
        self.anchored_elements.push(Self::new(type_id, element_id));
    }

    pub fn anchored_elements(&self) -> &Vec<AnchoredElement> { &self.anchored_elements }
    pub fn take_children(&mut self) -> Vec<AnchoredElement> { self.anchored_elements.drain(..).collect() }
    pub fn identifier(&self) -> &AnchorElementIdentifier { &self.identifier }
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

    pub fn trees(&self) -> Vec<&Vec<AnchoredElement>> {
        return vec![&self.screen_tree, &self.fixed_trees];
    }
    pub fn mut_trees(&mut self) -> Vec<&mut Vec<AnchoredElement>> {
        return vec![&mut self.screen_tree, &mut self.fixed_trees];
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
    pub fn add_screen_anchor(&mut self, type_id: TypeId, element_id: ResourceId<UiElementId>) {
        self.screen_tree.push(AnchoredElement::new(type_id, element_id));
    }

    /// Add anchor that is anchored to a fixed position on the screen
    pub fn add_fixed_anchor(&mut self, type_id: TypeId, element_id: ResourceId<UiElementId>) {
        self.fixed_trees.push(AnchoredElement::new(type_id, element_id));
    }

    /// Add anchor that is anchored to another element
    pub fn add_element_anchor(&mut self, anchor_type_id: TypeId, anchor_element_id: &ResourceId<UiElementId>, type_id: TypeId, element_id: ResourceId<UiElementId>)  -> Result<(), String>{
        match self.get_mut(anchor_type_id, anchor_element_id) {
            Some(anchor) => Ok(anchor.push(type_id, element_id)),
            None => {
                Err(format!("anchor with type {:?} and id {:?} was not found", anchor_type_id, anchor_element_id))
            },
        }
    }

    pub fn get(&self, type_id: TypeId, element_id: &ResourceId<UiElementId>) -> Option<&AnchoredElement> {
        for tree in self.trees().iter() {
            if let Some(element) = Self::get_from_tree(type_id, element_id.duplicate(), &tree) {
                return Some(element);
            }
        }

        None
    }

    fn get_from_tree(type_id: TypeId, element_id: ResourceId<UiElementId>, tree: &Vec<AnchoredElement>) -> Option<&AnchoredElement> {
        for entry in tree.iter() {
            match entry.get(type_id, &element_id) {
                Some(element) => return Some(element),
                None => (),
            }
        }

        None
    }

    pub fn get_mut(&mut self, type_id: TypeId, element_id: &ResourceId<UiElementId>) -> Option<&mut AnchoredElement> {
        for tree in self.mut_trees() {
            if let Some(element) = Self::get_mut_from_tree(type_id, element_id.duplicate(), tree) {
                return Some(element);
            }
        }

        None
    }

    fn get_mut_from_tree(type_id: TypeId, element_id: ResourceId<UiElementId>, tree: &mut Vec<AnchoredElement>) -> Option<&mut AnchoredElement> {
        for entry in tree {
            match entry.get_mut(type_id, &element_id) {
                Some(element) => return Some(element),
                None => (),
            }
        }

        None
    }

    pub fn get_by_id(&self, element_id: &ResourceId<UiElementId>) -> Option<&AnchoredElement> {
        for tree in self.trees() {
            if let Some(element) = Self::get_by_id_from_tree(element_id.duplicate(), tree) {
                return Some(element);
            }
        }

        None
    }

    fn get_by_id_from_tree(element_id: ResourceId<UiElementId>, tree: &Vec<AnchoredElement>) -> Option<&AnchoredElement> {
        for entry in tree.iter() {
            match entry.get_by_id(&element_id) {
                Some(element) => return Some(&element),
                None => (),
            }
        }

        None
    }

    fn get_parent_from_tree(child_id: ResourceId<UiElementId>, tree: &Vec<AnchoredElement>) -> Option<&AnchoredElement> {
        for root_element in tree.iter() {
            match root_element.get_parent(&child_id) {
                Some(parent_element) => return Some(&parent_element),
                None => (),
            }
        }

        None
    }

    fn get_mut_parent_from_tree(child_id: ResourceId<UiElementId>, tree: &mut Vec<AnchoredElement>) -> Option<&mut AnchoredElement> {
        for root_element in tree.iter_mut() {
            match root_element.get_parent_id(&child_id) {
                Some(parent_id) => {
                    return root_element.get_mut_by_id(&parent_id)
                },
                None => (),
            }
        }

        None
    }

    pub fn get_mut_by_id(&mut self, element_id: &ResourceId<UiElementId>) -> Option<&mut AnchoredElement> {
        for tree in self.mut_trees() {
            if let Some(element) = Self::get_mut_by_id_from_tree(element_id.duplicate(), tree) {
                return Some(element);
            }
        }

        None
    }

    fn get_mut_by_id_from_tree(element_id: ResourceId<UiElementId>, tree: &mut Vec<AnchoredElement>) -> Option<&mut AnchoredElement> {
        for entry in tree.iter_mut() {
            match entry.get_mut_by_id(&element_id) {
                Some(element) => return Some(element),
                None => (),
            }
        }

        None
    }

    /// Return a list of all elements child elements of a parent, in an order that can be used
    /// for updating the elements from parent to child
    pub fn get_children(&self, parent_id: &ResourceId<UiElementId>) -> Vec<AnchorElementIdentifier> {
        let mut result = vec![];

        match self.get_by_id(parent_id) {
            Some(parent) => {
                for child in parent.anchored_elements.iter() {
                    result.push(child.identifier.clone());
                    result.append(&mut self.get_children(&child.identifier.element_id));
                }
            },
            None => (),
        }

        result
    }

    /// * Returns `Some` if the parent was found
    /// * Returns `None` if the child is a root element, thus has no parent
    pub fn get_parent(&self, child_id: &ResourceId<UiElementId>) -> Option<&AnchoredElement> {
        for tree in self.trees() {
            if let Some(element) = Self::get_parent_from_tree(child_id.duplicate(), tree) {
                return Some(element);
            }
        }

        None
    }

    /// * Returns `Some` if the parent was found
    /// * Returns `None` if the child is a root element, thus has no parent
    pub fn get_mut_parent(&mut self, child_id: &ResourceId<UiElementId>) -> Option<&mut AnchoredElement> {
        for tree in self.mut_trees() {
            if let Some(element) = Self::get_mut_parent_from_tree(child_id.duplicate(), tree) {
                return Some(element);
            }
        }

        None
    }

    /// Returns wether the element was unregistered from the anchor tree:
    /// * `Some` - Contains the removed element, if the element was found
    /// * `None` - The element was not found, thus does not exist in the anchor tree
    pub fn remove_element_by_id(&mut self, element_id: &ResourceId<UiElementId>) -> Option<AnchoredElement> {
        for tree in self.mut_trees() {
            if let Some(removed_element) = Self::remove_tree_element_by_id(element_id, tree) {
                return Some(removed_element);
            } 
        }

        None
    }

    fn remove_tree_element_by_id(element_id: &ResourceId<UiElementId>, tree: &mut Vec<AnchoredElement>) -> Option<AnchoredElement> {
        for i in 0..tree.len() {
            if tree[i].identifier.element_id.equals(element_id) {
                // first we check if it's a root element that needs to be removed
                return Some(tree.remove(i));
            } else {
                // then we check if it's a child of a root element
                let mut removed_child = tree[i].remove_child_by_id(element_id);
                if removed_child.is_some() {
                    return removed_child.take();
                }
            }
        }

        None
    }

    /// Add children to the given parent.
    /// Returns an error of the parent (with id = `parent_id`) was not found
    pub fn add_children(&mut self, parent_id: &ResourceId<UiElementId>, mut children: Vec<AnchoredElement>) -> Result<(), String> {
        match self.get_mut_by_id(parent_id) {
            Some(parent) => Ok(parent.anchored_elements.append(&mut children)),
            None => Err(format!("parent with id {} was not found", parent_id.id())),
        }
    }

    pub fn root_screen_tree_elements(&self) -> Vec<AnchorElementIdentifier> {
        let mut result = vec![];

        for root_element in &self.screen_tree {
            result.push(root_element.identifier.clone());
        }

        result
    }
}

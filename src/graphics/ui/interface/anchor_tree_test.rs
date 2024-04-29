use std::any::TypeId;

use crate::{graphics::ui::element::ui_element_mock::UiElementMock, ResourceId};

use super::anchor_tree::AnchorTree;

#[test]
fn test_add_element_anchor() {
    let mut anchor_tree = AnchorTree::new();
    let type_id = TypeId::of::<UiElementMock>();

    let non_existing_id = ResourceId::new(0);
    let parent_id = ResourceId::new(1);
    let child_id = ResourceId::new(2);

    anchor_tree.add_fixed_anchor(type_id, parent_id);
    
    assert_eq!(true, anchor_tree.add_element_anchor(type_id, &non_existing_id, type_id, child_id).is_err());
    assert_eq!(true, anchor_tree.add_element_anchor(type_id, &parent_id, type_id, child_id).is_ok());
}

#[test]
/// Test the 'get' methods and all its variations using the following structure:
/// 
/// fixed tree: 
///     1
/// 
/// screen tree:
///     2 -> 4 -> 5
///     
///     3
/// 
fn test_get() -> Result<(), String> {
    let mut anchor_tree = AnchorTree::new();
    let type_id = TypeId::of::<UiElementMock>();
    let non_existing_type_id = TypeId::of::<String>();

    let non_existing_id = ResourceId::new(0);
    let fixed_parent_id = ResourceId::new(1);
    let screen_parent_id = ResourceId::new(2);
    let screen_parent_id_2 = ResourceId::new(3);
    let child_id = ResourceId::new(4);
    let nested_child = ResourceId::new(5);

    anchor_tree.add_fixed_anchor(type_id, screen_parent_id);
    anchor_tree.add_fixed_anchor(type_id, screen_parent_id_2);
    anchor_tree.add_fixed_anchor(type_id, fixed_parent_id);
    anchor_tree.add_element_anchor(type_id, &screen_parent_id, type_id, child_id.clone())?;
    anchor_tree.add_element_anchor(type_id, &child_id, type_id, nested_child)?;

    assert_eq!(true, anchor_tree.get(type_id, &non_existing_id).is_none());
    assert_eq!(true, anchor_tree.get(non_existing_type_id, &screen_parent_id).is_none());
    assert_eq!(true, anchor_tree.get(type_id, &fixed_parent_id).is_some());
    assert_eq!(true, anchor_tree.get(type_id, &screen_parent_id).is_some());
    assert_eq!(true, anchor_tree.get(type_id, &child_id).is_some());
    assert_eq!(true, anchor_tree.get(type_id, &nested_child).is_some());

    assert_eq!(true, anchor_tree.get_mut(type_id, &non_existing_id).is_none());
    assert_eq!(true, anchor_tree.get_mut(non_existing_type_id, &screen_parent_id).is_none());
    assert_eq!(true, anchor_tree.get_mut(type_id, &fixed_parent_id).is_some());
    assert_eq!(true, anchor_tree.get_mut(type_id, &screen_parent_id).is_some());
    assert_eq!(true, anchor_tree.get_mut(type_id, &child_id).is_some());
    assert_eq!(true, anchor_tree.get_mut(type_id, &nested_child).is_some());

    assert_eq!(true, anchor_tree.get_by_id(&non_existing_id).is_none());
    assert_eq!(true, anchor_tree.get_by_id(&fixed_parent_id).is_some());
    assert_eq!(true, anchor_tree.get_by_id(&screen_parent_id).is_some());
    assert_eq!(true, anchor_tree.get_by_id(&child_id).is_some());
    assert_eq!(true, anchor_tree.get_by_id(&nested_child).is_some());

    assert_eq!(true, anchor_tree.get_mut_by_id(&non_existing_id).is_none());
    assert_eq!(true, anchor_tree.get_mut_by_id(&fixed_parent_id).is_some());
    assert_eq!(true, anchor_tree.get_mut_by_id(&screen_parent_id).is_some());
    assert_eq!(true, anchor_tree.get_mut_by_id(&child_id).is_some());
    assert_eq!(true, anchor_tree.get_mut_by_id(&nested_child).is_some());

    Ok(())
}

#[test]
fn test_get_children() -> Result<(), String> {
    todo!()
}

#[test]
fn test_get_parent() -> Result<(), String> {
    todo!()
}

#[test]
fn test_remove_element_by_id() -> Result<(), String> {
    todo!()
}

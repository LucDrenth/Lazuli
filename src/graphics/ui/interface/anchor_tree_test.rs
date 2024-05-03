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
    
    assert!(anchor_tree.add_element_anchor(type_id, &non_existing_id, type_id, child_id).is_err());
    assert!(anchor_tree.add_element_anchor(type_id, &parent_id, type_id, child_id).is_ok());
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

    assert!(anchor_tree.get(type_id, &non_existing_id).is_none());
    assert!(anchor_tree.get(non_existing_type_id, &screen_parent_id).is_none());
    assert!(anchor_tree.get(type_id, &fixed_parent_id).is_some());
    assert!(anchor_tree.get(type_id, &screen_parent_id).is_some());
    assert!(anchor_tree.get(type_id, &child_id).is_some());
    assert!(anchor_tree.get(type_id, &nested_child).is_some());

    assert!(anchor_tree.get_mut(type_id, &non_existing_id).is_none());
    assert!(anchor_tree.get_mut(non_existing_type_id, &screen_parent_id).is_none());
    assert!(anchor_tree.get_mut(type_id, &fixed_parent_id).is_some());
    assert!(anchor_tree.get_mut(type_id, &screen_parent_id).is_some());
    assert!(anchor_tree.get_mut(type_id, &child_id).is_some());
    assert!(anchor_tree.get_mut(type_id, &nested_child).is_some());

    assert!(anchor_tree.get_by_id(&non_existing_id).is_none());
    assert!(anchor_tree.get_by_id(&fixed_parent_id).is_some());
    assert!(anchor_tree.get_by_id(&screen_parent_id).is_some());
    assert!(anchor_tree.get_by_id(&child_id).is_some());
    assert!(anchor_tree.get_by_id(&nested_child).is_some());

    assert!(anchor_tree.get_mut_by_id(&non_existing_id).is_none());
    assert!(anchor_tree.get_mut_by_id(&fixed_parent_id).is_some());
    assert!(anchor_tree.get_mut_by_id(&screen_parent_id).is_some());
    assert!(anchor_tree.get_mut_by_id(&child_id).is_some());
    assert!(anchor_tree.get_mut_by_id(&nested_child).is_some());

    Ok(())
}

#[test]
/// Test with the following tree:
///
///     1 --> 2 --> 3
///            \
///              --> 4 --> 5
/// 
fn test_get_children() -> Result<(), String> {
    let mut anchor_tree = AnchorTree::new();
    let type_id = TypeId::of::<UiElementMock>();

    let non_existing_id = ResourceId::new(0);
    let parent_id = ResourceId::new(1);
    let child = ResourceId::new(2);
    let child_nested_1 = ResourceId::new(3);
    let child_nested_2 = ResourceId::new(4);
    let child_nested_2_nested = ResourceId::new(5);

    anchor_tree.add_fixed_anchor(type_id, parent_id);
    anchor_tree.add_element_anchor(type_id, &parent_id, type_id, child)?;
    anchor_tree.add_element_anchor(type_id, &child, type_id, child_nested_1)?;
    anchor_tree.add_element_anchor(type_id, &child, type_id, child_nested_2)?;
    anchor_tree.add_element_anchor(type_id, &child_nested_2, type_id, child_nested_2_nested)?;

    assert_eq!(0, anchor_tree.get_children(&non_existing_id).len());
    assert_eq!(4, anchor_tree.get_children(&parent_id).len());
    assert_eq!(3, anchor_tree.get_children(&child).len());
    assert_eq!(1, anchor_tree.get_children(&child_nested_2).len());
    assert_eq!(0, anchor_tree.get_children(&child_nested_2_nested).len());

    Ok(())
}

#[test]
/// Test with the following tree:
///
///     1 --> 2 --> 3
///            \
///              --> 4 --> 5
/// 
fn test_get_parent() -> Result<(), String> {
    let mut anchor_tree = AnchorTree::new();
    let type_id = TypeId::of::<UiElementMock>();

    let non_existing_id = ResourceId::new(0);
    let parent_id = ResourceId::new(1);
    let child = ResourceId::new(2);
    let child_nested_1 = ResourceId::new(3);
    let child_nested_2 = ResourceId::new(4);
    let child_nested_2_nested = ResourceId::new(5);

    anchor_tree.add_fixed_anchor(type_id, parent_id);
    anchor_tree.add_element_anchor(type_id, &parent_id, type_id, child)?;
    anchor_tree.add_element_anchor(type_id, &child, type_id, child_nested_1)?;
    anchor_tree.add_element_anchor(type_id, &child, type_id, child_nested_2)?;
    anchor_tree.add_element_anchor(type_id, &child_nested_2, type_id, child_nested_2_nested)?;

    assert!(anchor_tree.get_parent(&non_existing_id).is_none());
    assert_eq!(4, *anchor_tree.get_parent(&child_nested_2_nested).unwrap().identifier().element_id.id());
    assert_eq!(2, *anchor_tree.get_parent(&child_nested_2).unwrap().identifier().element_id.id());
    assert_eq!(2, *anchor_tree.get_parent(&child_nested_1).unwrap().identifier().element_id.id());
    assert_eq!(1, *anchor_tree.get_parent(&child).unwrap().identifier().element_id.id());
    assert!(anchor_tree.get_parent(&parent_id).is_none());

    Ok(())
}

#[test]
/// Test with the following tree:
///
///     1 --> 2 --> 3
///            \
///              --> 4 --> 5
/// 
fn test_remove_element_by_id() -> Result<(), String> {
    let mut anchor_tree = AnchorTree::new();
    let type_id = TypeId::of::<UiElementMock>();

    let non_existing_id = ResourceId::new(0);
    let parent_id = ResourceId::new(1);
    let child = ResourceId::new(2);
    let child_nested_1 = ResourceId::new(3);
    let child_nested_2 = ResourceId::new(4);
    let child_nested_2_nested = ResourceId::new(5);

    anchor_tree.add_fixed_anchor(type_id, parent_id);
    anchor_tree.add_element_anchor(type_id, &parent_id, type_id, child)?;
    anchor_tree.add_element_anchor(type_id, &child, type_id, child_nested_1)?;
    anchor_tree.add_element_anchor(type_id, &child, type_id, child_nested_2)?;
    anchor_tree.add_element_anchor(type_id, &child_nested_2, type_id, child_nested_2_nested)?;

    assert!(anchor_tree.remove_element_by_id(&non_existing_id).is_none());
    assert!(anchor_tree.remove_element_by_id(&child_nested_2).unwrap().identifier().element_id.equals(&child_nested_2));
    assert!(anchor_tree.remove_element_by_id(&child_nested_2).is_none());

    Ok(())
}

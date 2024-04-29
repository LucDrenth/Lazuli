use crate::{graphics::ui::{element::ui_element_mock::UiElementMock, UiElementId}, ResourceId};

use super::element_list::ElementList;

#[test]
fn test_get_by_id() {
    let mut element_list: ElementList<UiElementMock> = ElementList::new();

    let existing_id = element_list.add(UiElementMock::default());
    let non_existing_id: ResourceId<UiElementId> = ResourceId::new(existing_id.id() + 1);

    assert_eq!(true, element_list.get_by_id(&existing_id).is_some());
    assert_eq!(true, element_list.get_by_id(&non_existing_id).is_none());
}

#[test]
fn test_get_mut_by_id() {
    let mut element_list: ElementList<UiElementMock> = ElementList::new();

    let existing_id = element_list.add(UiElementMock::default());
    let non_existing_id: ResourceId<UiElementId> = ResourceId::new(existing_id.id() + 1);

    assert_eq!(true, element_list.get_mut_by_id(&existing_id).is_some());
    assert_eq!(true, element_list.get_mut_by_id(&non_existing_id).is_none());
}

#[test]
fn test_get_by_index() {
    let mut element_list: ElementList<UiElementMock> = ElementList::new();

    let _ = element_list.add(UiElementMock::default());
    let existing_index: usize = 0;
    let non_existing_id: usize = 1;

    assert_eq!(true, element_list.get_by_index(existing_index).is_some());
    assert_eq!(true, element_list.get_by_index(non_existing_id).is_none());
}

#[test]
fn test_get_mut_by_index() {
    let mut element_list: ElementList<UiElementMock> = ElementList::new();

    let _ = element_list.add(UiElementMock::default());
    let existing_index: usize = 0;
    let non_existing_id: usize = 1;

    assert_eq!(true, element_list.get_mut_by_index(existing_index).is_some());
    assert_eq!(true, element_list.get_mut_by_index(non_existing_id).is_none());
}

#[test]
fn test_last() {
    let mut element_list: ElementList<UiElementMock> = ElementList::new();

    assert_eq!(true, element_list.last().is_none());

    let expected_id = 10;

    let mut last_element = UiElementMock::default();
    last_element.material_id = ResourceId::new(expected_id);

    let _ = element_list.add(UiElementMock::default());
    let _ = element_list.add(last_element);


    let last = element_list.last();
    assert_eq!(true, last.is_some());

    assert_eq!(expected_id, *last.unwrap().material_id.id());
}

#[test]
fn test_remove() {
    let mut element_list: ElementList<UiElementMock> = ElementList::new();
    let id_to_remove = element_list.add(UiElementMock::default());
    let non_existing_id: ResourceId<UiElementId> = ResourceId::new(id_to_remove.id() + 1);
    
    assert_eq!(false, element_list.remove(&non_existing_id));
    assert_eq!(true, element_list.remove(&id_to_remove));
    assert_eq!(false, element_list.remove(&id_to_remove)); // already removed, thus should be false
    assert_eq!(true, element_list.get_by_id(&id_to_remove).is_none());
}

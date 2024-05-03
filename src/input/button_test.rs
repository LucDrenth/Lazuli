use crate::input::button::ButtonRegistry;

use super::{input::InputElement, ButtonState};

const NUMBER_OF_BUTTONS: usize = 3;

#[derive(Copy, Clone, Debug)]
enum MockInputElement {
    Valid = 1,
    Invalid = 4, // higher than NUMBER_OF_BUTTONS
}

impl InputElement for MockInputElement {
    fn as_number(&self) -> usize {
        *self as usize
    }
}

#[test]
fn test_register_button_event() {
    let mut button_registry = ButtonRegistry::new(1);
    button_registry.register_button_event(&MockInputElement::Valid, ButtonState::Up);
    button_registry.register_button_event(&MockInputElement::Invalid, ButtonState::Down);
}

#[test]
fn test_is_button_down() {
    let mut button_registry = ButtonRegistry::new(NUMBER_OF_BUTTONS);

    assert_eq!(false, button_registry.is_button_down(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_down(&MockInputElement::Invalid));

    button_registry.register_button_event(&MockInputElement::Valid, ButtonState::Down);

    assert!(button_registry.is_button_down(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_down(&MockInputElement::Invalid));

    button_registry.reset();

    assert_eq!(false, button_registry.is_button_down(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_down(&MockInputElement::Invalid));
}

#[test]
fn test_is_button_up() {
    let mut button_registry = ButtonRegistry::new(NUMBER_OF_BUTTONS);

    assert_eq!(false, button_registry.is_button_up(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_up(&MockInputElement::Invalid));

    button_registry.register_button_event(&MockInputElement::Valid, ButtonState::Up);

    assert!(button_registry.is_button_up(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_up(&MockInputElement::Invalid));

    button_registry.reset();

    assert_eq!(false, button_registry.is_button_up(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_up(&MockInputElement::Invalid));
}

#[test]
fn test_is_button_held() {
    let mut button_registry = ButtonRegistry::new(NUMBER_OF_BUTTONS);

    assert_eq!(false, button_registry.is_button_held(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_held(&MockInputElement::Invalid));

    button_registry.register_button_event(&MockInputElement::Valid, ButtonState::Down);

    assert!(button_registry.is_button_held(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_held(&MockInputElement::Invalid));

    button_registry.reset();

    assert!(button_registry.is_button_held(&MockInputElement::Valid));
    assert_eq!(false, button_registry.is_button_held(&MockInputElement::Invalid));
}

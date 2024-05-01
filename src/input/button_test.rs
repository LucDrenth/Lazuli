use crate::input::button::{is_button_down, is_button_held, is_button_up, register_button_event};

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
    let mut current_state = [ButtonState::Up; NUMBER_OF_BUTTONS];
    assert_eq!(true, register_button_event(&mut current_state, &MockInputElement::Valid, ButtonState::Down).is_ok());
    assert_eq!(true, register_button_event(&mut current_state, &MockInputElement::Invalid, ButtonState::Down).is_err());
}

#[test]
fn test_is_button_down() -> Result<(), String> {
    let mut current_state = [ButtonState::Up; NUMBER_OF_BUTTONS];
    let last_state = [ButtonState::Up; NUMBER_OF_BUTTONS];

    assert_eq!(false, is_button_down(&current_state, &last_state, &MockInputElement::Valid));
    assert_eq!(false, is_button_down(&current_state, &last_state, &MockInputElement::Invalid));

    register_button_event(&mut current_state, &MockInputElement::Valid, ButtonState::Down)?;
    assert_eq!(true, is_button_down(&current_state, &last_state, &MockInputElement::Valid));

    Ok(())
}

#[test]
fn test_is_button_up() -> Result<(), String> {
    let mut current_state = [ButtonState::Down; NUMBER_OF_BUTTONS];
    let last_state = [ButtonState::Down; NUMBER_OF_BUTTONS];

    assert_eq!(false, is_button_up(&current_state, &last_state, &MockInputElement::Valid));
    assert_eq!(false, is_button_up(&current_state, &last_state, &MockInputElement::Invalid));

    register_button_event(&mut current_state, &MockInputElement::Valid, ButtonState::Up)?;
    assert_eq!(true, is_button_up(&current_state, &last_state, &MockInputElement::Valid));

    Ok(())
}

#[test]
fn test_is_button_held() -> Result<(), String> {
    let mut current_state = [ButtonState::Up; NUMBER_OF_BUTTONS];

    assert_eq!(false, is_button_held(&current_state, &MockInputElement::Valid));
    assert_eq!(false, is_button_held(&current_state, &MockInputElement::Invalid));

    register_button_event(&mut current_state, &MockInputElement::Valid, ButtonState::Down)?;
    assert_eq!(true, is_button_held(&current_state, &MockInputElement::Valid));

    Ok(())
}

use super::input::InputElement;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ButtonState {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    Down,
    Up,
    UpOrDown,
}

pub fn is_button_down(current_state: &[ButtonState], last_state: &[ButtonState], input_element: &dyn InputElement) -> bool {
    let element_number = input_element.as_number();

    if element_number >= current_state.len() || element_number >= last_state.len() {
        return false;
    }

    return current_state[element_number] == ButtonState::Down 
        && last_state[element_number] == ButtonState::Up;
}


pub fn is_button_up(current_state: &[ButtonState], last_state: &[ButtonState], input_element: &dyn InputElement) -> bool {
    let element_number = input_element.as_number();

    if element_number >= current_state.len() || element_number >= last_state.len() {
        return false;
    }

    return current_state[element_number] == ButtonState::Up 
        && last_state[element_number] == ButtonState::Down;
}

pub fn is_button_held(current_state: &[ButtonState], input_element: &dyn InputElement) -> bool {
    let element_number = input_element.as_number();

    if element_number >= current_state.len() {
        return false;
    }

    return current_state[element_number] == ButtonState::Down 
}

pub fn is_action(current_state: &[ButtonState], last_state: &[ButtonState], input_element: &dyn InputElement, action: &ButtonAction) -> bool {
    match action {
        ButtonAction::Down => is_button_down(current_state, last_state, input_element),
        ButtonAction::Up => is_button_up(current_state, last_state, input_element),
        ButtonAction::UpOrDown => is_button_up(current_state, last_state, input_element) || is_button_down(current_state, last_state, input_element),
    }
}

pub fn register_button_event(current_state: &mut [ButtonState], input_element: &dyn InputElement, state: ButtonState) -> Result<(), String> {
    let element_number = input_element.as_number();

    if element_number >= current_state.len() {
        return Err(format!("element number {} exceeds state array of length {}", element_number, current_state.len()));
    }

    current_state[element_number] = state;

    Ok(())
}

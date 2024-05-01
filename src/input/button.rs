use super::input::InputElement;

pub struct ButtonRegistry {
    /// button events that are registered before the current app tick and after the last app tick
    registered_button_events: Vec<ButtonEvents>,
    /// The most recent recorded state of buttons
    current_button_states: Vec<ButtonState>,
}

impl ButtonRegistry {
    pub fn new(capacity: usize) -> Self {
        Self {
            registered_button_events: Vec::with_capacity(capacity),
            current_button_states: Vec::with_capacity(capacity),
        }
    }

    pub fn reset(&mut self) {
        for button_events in &mut self.registered_button_events {
            button_events.reset();
        }
    }

    pub fn register_button_event(&mut self, button: &dyn InputElement, state: ButtonState) {
        self.ensure_current_button_states_length(button);
        self.ensure_registered_button_events_length(button);

        self.current_button_states[button.as_number()] = state;
        self.registered_button_events[button.as_number()].register(state);
    }

    fn ensure_current_button_states_length(&mut self, button: &dyn InputElement) {
        if button.as_number() >= self.current_button_states.len() {
            for _ in self.current_button_states.len()..=button.as_number() {
                self.current_button_states.push(ButtonState::Up)
            }
        }
    }

    fn ensure_registered_button_events_length(&mut self, button: &dyn InputElement) {
        if button.as_number() >= self.registered_button_events.len() {
            for _ in self.registered_button_events.len()..=button.as_number() {
                self.registered_button_events.push(ButtonEvents::new())
            }
        }
    }

    pub fn is_button_down(&self, button: &dyn InputElement) -> bool {
        if button.as_number() >= self.registered_button_events.len() {
            return false;
        }
        
        self.registered_button_events[button.as_number()].is_down()
    }

    pub fn is_button_up(&self, button: &dyn InputElement) -> bool {
        if button.as_number() >= self.registered_button_events.len() {
            return false;
        }

        self.registered_button_events[button.as_number()].is_up()
    }

    pub fn is_button_held(&self, button: &dyn InputElement) -> bool {
        if button.as_number() >= self.current_button_states.len() {
            return false;
        }

        self.current_button_states[button.as_number()] == ButtonState::Down
    }

    pub fn is_button_action(&self, button: &dyn InputElement, action: &ButtonAction) -> bool {
        match action {
            ButtonAction::Down => self.is_button_down(button),
            ButtonAction::Up => self.is_button_up(button),
            ButtonAction::UpOrDown => self.is_button_down(button) || self.is_button_up(button),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ButtonState {
    Up,
    Down,
}

pub struct ButtonEvents {
    is_up: bool,
    is_down: bool,
}

impl ButtonEvents {
    pub fn new() -> Self {
        Self { is_up: false, is_down: false }
    }
    
    pub fn reset(&mut self) {
        self.is_up = false;
        self.is_down = false;
    }

    pub fn register(&mut self, state: ButtonState) {
        match state {
            ButtonState::Up => self.is_up = true,
            ButtonState::Down => self.is_down = true,
        }
    }

    pub fn is_up(&self) -> bool {
        self.is_up
    }
    
    pub fn is_down(&self) -> bool {
        self.is_down
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    Down,
    Up,
    UpOrDown,
}

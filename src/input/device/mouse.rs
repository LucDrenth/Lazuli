use glam::Vec2;

use crate::{input::{button, input::InputElement, ButtonAction, ButtonState}, log};

pub struct MouseDevice {
    current_state: State,
    last_state: State,
}

impl MouseDevice {
    pub fn new () -> Self {
        Self {
            current_state: State::new(),
            last_state: State::new(),
        }
    }

    pub fn reset(&mut self) {
        self.last_state = self.current_state;
        self.current_state.reset();
    }

    pub fn register_button_event(&mut self, button: MouseButton, state: ButtonState) {
        log::info(format!("register mouse button {:?} with state {:?}", button, state));

        button::register_button_event(&mut self.current_state.buttons, &button, state).unwrap_or_else(|err|{
            log::engine_warn(format!("Failed to register mouse button event {:?} with state {:?}: {}", button, state, err));
        });
    }

    pub fn register_scroll_x_event(&mut self, scroll: f64) {
        self.current_state.scroll_x += scroll;
    }

    pub fn register_scroll_y_event(&mut self, scroll: f64) {
        self.current_state.scroll_y += scroll;
    }

    pub fn register_reposition_event(&mut self, position_x: f64, position_y: f64) {
        self.current_state.position_x = position_x;
        self.current_state.position_y = position_y;
    }

    pub fn register_move_event(&mut self, moved_x: f64, moved_y: f64) {
        self.current_state.moved_x += moved_x;
        self.current_state.moved_y += moved_y;
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        button::is_button_down(&self.current_state.buttons, &self.last_state.buttons, &button)
    }

    pub fn is_button_up(&self, button: MouseButton) -> bool {
        button::is_button_up(&self.current_state.buttons, &self.last_state.buttons, &button)
    }

    pub fn is_button_held(&self, button: MouseButton) -> bool {
        button::is_button_held(&self.current_state.buttons, &button)
    }

    pub fn is_button_action(&self, mouse_button: MouseButton, action: &ButtonAction) -> bool {
        button::is_action(&self.current_state.buttons, &self.last_state.buttons, &mouse_button, &action)
    }

    pub fn get_position_x(&self) -> f64 {
        self.current_state.position_x
    }

    pub fn get_position_y(&self) -> f64 {
        self.current_state.position_y
    }

    pub fn get_position(&self) -> Vec2 {
        Vec2 { x: self.get_position_x() as f32, y: self.get_position_y() as f32 }
    }

    pub fn get_moved_x(&self) -> f64 {
        self.current_state.moved_x
    }

    pub fn get_moved_y(&self) -> f64 {
        self.current_state.moved_y
    }

    pub fn get_moved(&self) -> Vec2 {
        Vec2 { x: self.get_moved_x() as f32, y: self.get_moved_y() as f32 }
    }

    pub fn did_move(&self) -> bool {
        self.get_moved_x() != 0.0 || self.get_moved_y() != 0.0
    }

    pub fn get_scroll_x(&self) -> f64 {
        self.current_state.scroll_x
    }

    pub fn get_scroll_y(&self) -> f64 {
        self.current_state.scroll_y
    }

    pub fn has_scroll(&self) -> bool {
        self.current_state.scroll_x != 0.0 && self.current_state.scroll_y != 0.0
    }
}

#[derive(Copy, Clone)]
struct State {
    buttons: [ButtonState; 16],
    scroll_x: f64,
    scroll_y: f64,
    moved_x: f64,
    moved_y: f64,
    position_x: f64,
    position_y: f64,
}

impl State {
    fn new() -> Self {
        Self {
            buttons: [ButtonState::Up; 16],
            scroll_x: 0.0,
            scroll_y: 0.0,
            moved_x: 0.0,
            moved_y: 0.0,
            position_x: 0.0,
            position_y: 0.0,
        }
    }

    fn reset(&mut self) {
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
        self.moved_x = 0.0;
        self.moved_y = 0.0;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Uknown = 0,
    Left = 1,
    Right = 2,
    Middle = 3, // usually the scrolwheel
    WheelLeft = 4, // not all mices have this button
    WheelRight = 5, // not all mices have this button
    Extra1 = 6,
    Extra2 = 7,
    Extra3 = 8,
    Extra4 = 9,
}

impl InputElement for MouseButton {
    fn as_number(&self) -> usize {
        *self as usize
    }
}

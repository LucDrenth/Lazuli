use glam::Vec2;

pub struct Input {
    last_state: State,
    current_state: State,
}

pub enum InputAction {
    Down,
    Up,
    UpOrDown,
}

impl Input {
    pub fn new() -> Self {
        return Input { 
            last_state: State::new(),
            current_state: State::new(),
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        let key_number = key.as_number();

        return self.current_state.keys[key_number] == KeyState::Down 
            && self.last_state.keys[key_number] == KeyState::Up
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        let key_number = key.as_number();
        
        return self.current_state.keys[key_number] == KeyState::Up 
            && self.last_state.keys[key_number] == KeyState::Down
    }

    pub fn is_key_held(&self, key: Key) -> bool {
        let key_number = key.as_number();
        
        return self.current_state.keys[key_number] == KeyState::Down;
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        let button_number = button.as_number();

        return self.current_state.mouse_buttons[button_number] == MouseButtonState::Down 
            && self.last_state.mouse_buttons[button_number] == MouseButtonState::Up
    }

    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        let button_number = button.as_number();

        return self.current_state.mouse_buttons[button_number] == MouseButtonState::Up 
            && self.last_state.mouse_buttons[button_number] == MouseButtonState::Down
    }

    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        let button_number = button.as_number();

        return self.current_state.mouse_buttons[button_number] == MouseButtonState::Down;
    }

    pub fn is_mouse_button_action(&self, mouse_button: MouseButton, action: &InputAction) -> bool {
        match action {
            InputAction::Down => self.is_mouse_button_down(mouse_button),
            InputAction::Up => self.is_mouse_button_up(mouse_button),
            InputAction::UpOrDown => self.is_mouse_button_up(mouse_button) || self.is_mouse_button_down(mouse_button),
        }
    }

    pub fn get_mouse_position_x(&self) -> f64 {
        self.current_state.mouse_x
    }

    pub fn get_mouse_position_y(&self) -> f64 {
        self.current_state.mouse_y
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        Vec2 { x: self.get_mouse_position_x() as f32, y: self.get_mouse_position_y() as f32 }
    }

    pub fn get_mouse_moved_x(&self) -> f64 {
        self.current_state.mouse_moved_x
    }

    pub fn get_mouse_moved_y(&self) -> f64 {
        self.current_state.mouse_moved_y
    }

    pub fn get_mouse_moved(&self) -> Vec2 {
        Vec2 { x: self.get_mouse_moved_x() as f32, y: self.get_mouse_moved_y() as f32 }
    }

    pub fn did_mouse_move(&self) -> bool {
        self.get_mouse_moved_x() != 0.0 || self.get_mouse_moved_y() != 0.0
    }

    pub fn get_scroll_x(&self) -> f64 {
        self.current_state.scroll_x
    }

    pub fn get_scroll_y(&self) -> f64 {
        self.current_state.scroll_y
    }


    pub fn register_key_event(&mut self, key: Key, state: KeyState) {
        self.current_state.keys[key.as_number()] = state;
    }

    pub fn register_mouse_button_event(&mut self, button: MouseButton, state: MouseButtonState) {
        self.current_state.mouse_buttons[button.as_number()] = state;
    }

    pub fn register_scroll_x_event(&mut self, scroll: f64) {
        self.current_state.scroll_x += scroll;
    }

    pub fn register_scroll_y_event(&mut self, scroll: f64) {
        self.current_state.scroll_y += scroll;
    }

    pub fn register_mouse_reposition_event(&mut self, position_x: f64, position_y: f64) {
        self.current_state.mouse_x = position_x;
        self.current_state.mouse_y = position_y;
    }

    pub fn register_mouse_move_event(&mut self, moved_x: f64, moved_y: f64) {
        self.current_state.mouse_moved_x += moved_x;
        self.current_state.mouse_moved_y += moved_y;
    }


    pub fn reset(&mut self) {
        self.last_state = self.current_state;
        self.current_state.scroll_x = 0.0;
        self.current_state.scroll_y = 0.0;
        self.current_state.mouse_moved_x = 0.0;
        self.current_state.mouse_moved_y = 0.0;
    }
}

#[derive(Copy, Clone)]
struct State {
    keys: [KeyState; 256],
    mouse_buttons: [MouseButtonState; 16],
    scroll_x: f64,
    scroll_y: f64,
    mouse_moved_x: f64,
    mouse_moved_y: f64,
    mouse_x: f64,
    mouse_y: f64,
}

impl State {
    fn new() -> Self {
        Self {
            keys: [KeyState::Up; 256],
            mouse_buttons: [MouseButtonState::Up; 16],
            scroll_x: 0.0,
            scroll_y: 0.0,
            mouse_moved_x: 0.0,
            mouse_moved_y: 0.0,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    Up, // pressed down
    Down, // released
}

#[derive(Copy, Clone)]
pub enum Key {
    Uknown = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
    N = 14,
    O = 15,
    P = 16,
    Q = 17,
    R = 18,
    S = 19,
    T = 20,
    U = 21,
    V = 22,
    W = 23,
    X = 24,
    Y = 25,
    Z = 26,

    One = 27,
    Two = 28,
    Three = 29,
    Four = 30,
    Five = 31,
    Six = 32,
    Seven = 33,
    Eight = 34,
    Nine = 35,
    Zero = 36,

    F1 = 37,
    F2 = 38,
    F3 = 39,
    F4 = 40,
    F5 = 41,
    F6 = 42,
    F7 = 43,
    F8 = 44,
    F9 = 45,
    F10 = 46,
    F11 = 47,
    F12 = 48,

    Shift = 49,
    Escape = 50,
    Tab = 51,
    Cntrl = 52,
    Alt = 53,
    Space = 54,

    ArrowLeft = 55,
    ArrowRight = 56,
    ArrowUp = 57,
    ArrowDown = 58,

    Cmd = 59,
    BackTick = 60,
    Enter = 61,
}

impl Key {
    pub fn as_number(&self) -> usize {
        *self as usize
    }
}


#[derive(Copy, Clone, PartialEq)]
pub enum MouseButtonState {
    Up, // pressed down
    Down, // released
}

#[derive(Copy, Clone)]
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

impl MouseButton {
    pub fn as_number(&self) -> usize {
        *self as usize
    }
}

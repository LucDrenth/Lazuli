use crate::log;

pub struct KeyboardDevice {
    current_state: State,
    last_state: State,
}

impl KeyboardDevice {
    pub fn new () -> Self {
        Self { 
            current_state: State::new(), 
            last_state: State::new(),
        }
    }

    pub fn reset(&mut self) {
        self.last_state = self.current_state;
    }

    pub fn register_key_event(&mut self, key: Key, state: KeyState) {
        if key.as_number() > self.current_state.keys.len() {
            log::engine_warn(format!(
                "Tried to register key event [{:?}] with state [{:?}] but its number representation [{}] does not fit in self.current_state.keys, which has length [{}]", 
                key, state, key.as_number(), self.current_state.keys.len()
            ));
            return;
        }
        
        self.current_state.keys[key.as_number()] = state;
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
}

#[derive(Copy, Clone)]
struct State {
    keys: [KeyState; 256],
}

impl State {
    fn new() -> Self {
        Self {
            keys: [KeyState::Up; 256],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyState {
    Up, // pressed down
    Down, // released
}

#[derive(Copy, Clone, Debug)]
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
    Backspace = 62,
    Equals = 63,
    Minus = 64,
    Plus = 65,
    Caret = 66,
}

impl Key {
    pub fn as_number(&self) -> usize {
        *self as usize
    }
}

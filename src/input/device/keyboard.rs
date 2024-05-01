use button::ButtonAction;

use crate::input::{button::{self, ButtonRegistry}, input::InputElement, ButtonState};

pub struct KeyboardDevice {
    key_registry: ButtonRegistry,
}

impl KeyboardDevice {
    pub fn new () -> Self {
        Self { 
            key_registry: ButtonRegistry::new(256),
        }
    }

    pub fn reset(&mut self) {
        self.key_registry.reset();
    }

    pub fn register_key_event(&mut self, key: Key, state: ButtonState) {
        self.key_registry.register_button_event(&key, state);
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.key_registry.is_button_down(&key)
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        self.key_registry.is_button_up(&key)
    }

    pub fn is_key_held(&self, key: Key) -> bool {        
        self.key_registry.is_button_held(&key)
    }

    pub fn is_key_action(&self, key: Key, action: &ButtonAction) -> bool {
        self.key_registry.is_button_action(&key, action)
    }
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

impl InputElement for Key {
    fn as_number(&self) -> usize {
        *self as usize
    }
}

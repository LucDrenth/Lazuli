use super::{KeyboardDevice, MouseDevice};

pub struct Input {
    pub mouse: MouseDevice,
    pub keyboard: KeyboardDevice,
}

impl Input {
    pub fn new() -> Self {
        return Input { 
            mouse: MouseDevice::new(),
            keyboard: KeyboardDevice::new(),
        }
    }

    pub fn reset(&mut self) {
        self.mouse.reset();
        self.keyboard.reset();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    Down,
    Up,
    UpOrDown,
}

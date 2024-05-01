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

pub trait InputElement {
    fn as_number(&self) -> usize;
}

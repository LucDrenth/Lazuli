mod input;
pub use input::Input;

mod device;
pub use device::*;

mod button;
pub use button::ButtonAction;
pub use button::ButtonState;

#[cfg(test)]
pub mod button_test;

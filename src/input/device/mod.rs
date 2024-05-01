mod keyboard;
pub use keyboard::KeyboardDevice;
pub use keyboard::Key;

mod mouse;
pub use mouse::MouseDevice;
pub use mouse::MouseButton;

#[cfg(test)]
pub mod keyboard_test;
#[cfg(test)]
pub mod mouse_test;

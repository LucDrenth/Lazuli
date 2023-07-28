/// Functions to map input from the "glutin" crate to that of our own

use crate::{input::{Key, KeyState, MouseButton, MouseButtonState}, log};

pub fn map_glutin_keycode(glutin_key: glutin::event::VirtualKeyCode) -> Key {
    match glutin_key {
        glutin::event::VirtualKeyCode::Key1 => Key::One,
        glutin::event::VirtualKeyCode::Key2 => Key::Two,
        glutin::event::VirtualKeyCode::Key3 => Key::Three,
        glutin::event::VirtualKeyCode::Key4 => Key::Four,
        glutin::event::VirtualKeyCode::Key5 => Key::Five,
        glutin::event::VirtualKeyCode::Key6 => Key::Six,
        glutin::event::VirtualKeyCode::Key7 => Key::Seven,
        glutin::event::VirtualKeyCode::Key8 => Key::Eight,
        glutin::event::VirtualKeyCode::Key9 => Key::Nine,
        glutin::event::VirtualKeyCode::Key0 => Key::Zero,
        glutin::event::VirtualKeyCode::A => Key::A,
        glutin::event::VirtualKeyCode::B => Key::B,
        glutin::event::VirtualKeyCode::C => Key::C,
        glutin::event::VirtualKeyCode::D => Key::D,
        glutin::event::VirtualKeyCode::E => Key::E,
        glutin::event::VirtualKeyCode::F => Key::F,
        glutin::event::VirtualKeyCode::G => Key::G,
        glutin::event::VirtualKeyCode::H => Key::H,
        glutin::event::VirtualKeyCode::I => Key::I,
        glutin::event::VirtualKeyCode::J => Key::J,
        glutin::event::VirtualKeyCode::K => Key::K,
        glutin::event::VirtualKeyCode::L => Key::L,
        glutin::event::VirtualKeyCode::M => Key::M,
        glutin::event::VirtualKeyCode::N => Key::N,
        glutin::event::VirtualKeyCode::O => Key::O,
        glutin::event::VirtualKeyCode::P => Key::P,
        glutin::event::VirtualKeyCode::Q => Key::Q,
        glutin::event::VirtualKeyCode::R => Key::R,
        glutin::event::VirtualKeyCode::S => Key::S,
        glutin::event::VirtualKeyCode::T => Key::T,
        glutin::event::VirtualKeyCode::U => Key::U,
        glutin::event::VirtualKeyCode::V => Key::V,
        glutin::event::VirtualKeyCode::W => Key::W,
        glutin::event::VirtualKeyCode::X => Key::X,
        glutin::event::VirtualKeyCode::Y => Key::Y,
        glutin::event::VirtualKeyCode::Z => Key::Z,
        glutin::event::VirtualKeyCode::Escape => Key::Escape,
        glutin::event::VirtualKeyCode::F1 => Key::F1,
        glutin::event::VirtualKeyCode::F2 => Key::F2,
        glutin::event::VirtualKeyCode::F3 => Key::F3,
        glutin::event::VirtualKeyCode::F4 => Key::F4,
        glutin::event::VirtualKeyCode::F5 => Key::F5,
        glutin::event::VirtualKeyCode::F6 => Key::F6,
        glutin::event::VirtualKeyCode::F7 => Key::F7,
        glutin::event::VirtualKeyCode::F8 => Key::F8,
        glutin::event::VirtualKeyCode::F9 => Key::F9,
        glutin::event::VirtualKeyCode::F10 => Key::F10,
        glutin::event::VirtualKeyCode::F11 => Key::F11,
        glutin::event::VirtualKeyCode::F12 => Key::F12,
        glutin::event::VirtualKeyCode::Left => Key::ArrowLeft,
        glutin::event::VirtualKeyCode::Up => Key::ArrowUp,
        glutin::event::VirtualKeyCode::Right => Key::ArrowRight,
        glutin::event::VirtualKeyCode::Down => Key::ArrowDown,
        glutin::event::VirtualKeyCode::Space => Key::Space,
        glutin::event::VirtualKeyCode::Numpad0 => Key::Zero,
        glutin::event::VirtualKeyCode::Numpad1 => Key::One,
        glutin::event::VirtualKeyCode::Numpad2 => Key::Two,
        glutin::event::VirtualKeyCode::Numpad3 => Key::Three,
        glutin::event::VirtualKeyCode::Numpad4 => Key::Four,
        glutin::event::VirtualKeyCode::Numpad5 => Key::Five,
        glutin::event::VirtualKeyCode::Numpad6 => Key::Six,
        glutin::event::VirtualKeyCode::Numpad7 => Key::Seven,
        glutin::event::VirtualKeyCode::Numpad8 => Key::Eight,
        glutin::event::VirtualKeyCode::Numpad9 => Key::Nine,
        glutin::event::VirtualKeyCode::LShift => Key::Shift,
        glutin::event::VirtualKeyCode::LControl => Key::Cntrl,
        glutin::event::VirtualKeyCode::LWin => Key::Cmd,
        glutin::event::VirtualKeyCode::LAlt => Key::Alt,
        glutin::event::VirtualKeyCode::Tab => Key::Tab,
        glutin::event::VirtualKeyCode::Grave => Key::BackTick,
        glutin::event::VirtualKeyCode::Return => Key::Enter,
        _ => {
            log::engine_warn(format!("Unhandled glutin key event: {:?}", glutin_key));
            Key::Uknown
        },
    }
}

pub fn map_glutin_key_state(state: glutin::event::ElementState) -> KeyState {
    match state {
        glutin::event::ElementState::Pressed => KeyState::Down,
        glutin::event::ElementState::Released => KeyState::Up,
    }
}

pub fn map_glutin_mouse_button(state: glutin::event::MouseButton) -> MouseButton {
    match state {
        glutin::event::MouseButton::Left => MouseButton::Left,
        glutin::event::MouseButton::Right => MouseButton::Right,
        glutin::event::MouseButton::Middle => MouseButton::Middle,
        glutin::event::MouseButton::Other(val) => {
            log::engine_warn(format!("Unhandled glutin mouse button event: other[{}]", val));
            MouseButton::Uknown
        },
    }
}

pub fn map_glutin_mouse_button_state(state: glutin::event::ElementState) -> MouseButtonState {
    match state {
        glutin::event::ElementState::Pressed => MouseButtonState::Down,
        glutin::event::ElementState::Released => MouseButtonState::Up,
    }
}

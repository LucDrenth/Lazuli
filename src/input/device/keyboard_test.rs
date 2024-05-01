use crate::input::{ButtonState, Key, KeyboardDevice};

#[test]
fn test_is_key_down() {
    let mut keyboard = KeyboardDevice::new();
    let key = Key::L;

    assert_eq!(false, keyboard.is_key_down(key));

    keyboard.register_key_event(key, ButtonState::Down);
    assert_eq!(true, keyboard.is_key_down(key));

    // next frame
    keyboard.reset();

    keyboard.register_key_event(key, ButtonState::Up);
    assert_eq!(false, keyboard.is_key_down(key));
    
    // next frame
    keyboard.reset();
    
    assert_eq!(false, keyboard.is_key_down(key));

    // next frame
    keyboard.reset();

    keyboard.register_key_event(key, ButtonState::Down);
    keyboard.register_key_event(key, ButtonState::Up);
    assert_eq!(true, keyboard.is_key_down(key));

    // next frame
    keyboard.reset();

    keyboard.register_key_event(key, ButtonState::Down);
    keyboard.register_key_event(key, ButtonState::Up);
    keyboard.register_key_event(key, ButtonState::Down);
    assert_eq!(true, keyboard.is_key_down(key));
}

#[test]
fn test_is_key_up() {
    let mut keyboard = KeyboardDevice::new();
    let key = Key::L;

    assert_eq!(false, keyboard.is_key_up(key));

    keyboard.register_key_event(key, ButtonState::Down);
    assert_eq!(false, keyboard.is_key_up(key));

    // next frame
    keyboard.reset();

    keyboard.register_key_event(key, ButtonState::Up);
    assert_eq!(true, keyboard.is_key_up(key));

    // next frame
    keyboard.reset();

    assert_eq!(false, keyboard.is_key_up(key));
}

#[test]
fn test_is_key_held() {
    let mut keyboard = KeyboardDevice::new();
    let key = Key::L;

    assert_eq!(false, keyboard.is_key_held(key));

    keyboard.register_key_event(key, ButtonState::Down);
    assert_eq!(true, keyboard.is_key_held(key));

    // next frame
    keyboard.reset();
    assert_eq!(true, keyboard.is_key_held(key));

    keyboard.register_key_event(key, ButtonState::Up);
    assert_eq!(false, keyboard.is_key_held(key));
}

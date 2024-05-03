use crate::input::{ButtonState, MouseButton, MouseDevice};

#[test]
fn test_is_button_down() {
    let mut mouse = MouseDevice::new();
    let button = MouseButton::Left;

    assert_eq!(false, mouse.is_button_down(button));

    mouse.register_button_event(button, ButtonState::Down);
    assert!(mouse.is_button_down(button));

    // next frame
    mouse.reset();

    mouse.register_button_event(button, ButtonState::Up);
    assert_eq!(false, mouse.is_button_down(button));
    
    // next frame
    mouse.reset();
    
    assert_eq!(false, mouse.is_button_down(button));

    // next frame
    mouse.reset();

    mouse.register_button_event(button, ButtonState::Down);
    mouse.register_button_event(button, ButtonState::Up);
    assert!(mouse.is_button_down(button));

    // next frame
    mouse.reset();

    mouse.register_button_event(button, ButtonState::Down);
    mouse.register_button_event(button, ButtonState::Up);
    mouse.register_button_event(button, ButtonState::Down);
    assert!(mouse.is_button_down(button));
}

#[test]
fn test_is_button_up() {
    let mut mouse = MouseDevice::new();
    let button = MouseButton::Left;

    assert_eq!(false, mouse.is_button_up(button));

    mouse.register_button_event(button, ButtonState::Down);
    assert_eq!(false, mouse.is_button_up(button));

    // next frame
    mouse.reset();

    mouse.register_button_event(button, ButtonState::Up);
    assert!(mouse.is_button_up(button));

    // next frame
    mouse.reset();

    assert_eq!(false, mouse.is_button_up(button));

    // next frame
    mouse.reset();

    mouse.register_button_event(button, ButtonState::Down);
    mouse.register_button_event(button, ButtonState::Up);
    assert!(mouse.is_button_up(button));
}

#[test]
fn test_is_button_held() {
    let mut mouse = MouseDevice::new();
    let button = MouseButton::Left;

    assert_eq!(false, mouse.is_button_held(button));

    mouse.register_button_event(button, ButtonState::Down);
    assert!(mouse.is_button_held(button));

    // next frame
    mouse.reset();
    assert!(mouse.is_button_held(button));

    mouse.register_button_event(button, ButtonState::Up);
    assert_eq!(false, mouse.is_button_held(button));

    // next frame
    mouse.reset();

    mouse.register_button_event(button, ButtonState::Down);
    mouse.register_button_event(button, ButtonState::Up);
    assert_eq!(false, mouse.is_button_held(button));
}

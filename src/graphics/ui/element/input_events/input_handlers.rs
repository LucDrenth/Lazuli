use crate::{input::{Input, InputAction, MouseButton}, log};

use super::{drag_event_handler::DragEventHandler, EventHandler};

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    MouseLeftDown,
    MouseLeftUp,
    MouseLeftDrag,
    Hover,
    Scroll,
}

// TODO make mouse handlers dynamic so that we don't have to copy over if we want
// handlers for other mouse buttons
#[derive(Clone, Copy)]
pub struct InputEventHandlers {
    pub mouse_left_down_handler: EventHandler,
    pub mouse_left_up_handler: EventHandler,
    pub mouse_left_drag_handler: DragEventHandler,
    pub hover_handler: EventHandler,
    pub scroll_handler: EventHandler,
}

impl InputEventHandlers {
    pub fn new() -> Self {
        Self {
            mouse_left_down_handler: EventHandler::new(true, true),
            mouse_left_up_handler: EventHandler::new(true, true),
            mouse_left_drag_handler: DragEventHandler::new(true, true),
            hover_handler: EventHandler::new(true, true),
            scroll_handler: EventHandler::new(false, true),
        }
    }

    /// Returns true if the event has been handled and captured
    pub fn register_event(&mut self, event: InputEvent, input: &Input) -> bool {
        let handler: &mut EventHandler = match event {
            InputEvent::MouseLeftUp => &mut self.mouse_left_up_handler,
            InputEvent::MouseLeftDown => &mut self.mouse_left_down_handler,
            InputEvent::MouseLeftDrag => return self.mouse_left_drag_handler.register(input),
            InputEvent::Hover => &mut self.hover_handler,
            InputEvent::Scroll => &mut self.scroll_handler,
        };

        handler.try_to_handle();
        handler.did_capture()
    }

    pub fn reset(&mut self, input: &Input) {
        self.mouse_left_down_handler.reset();
        self.mouse_left_up_handler.reset();
        self.mouse_left_drag_handler.reset(input);
        self.hover_handler.reset();
        self.scroll_handler.reset();
    }

    pub fn did_handle_mouse_event(&self, mouse_button: &MouseButton, input_action: &InputAction) -> bool {
        if matches!(mouse_button, MouseButton::Left) && matches!(input_action, InputAction::Down) {
            return self.mouse_left_down_handler.did_handle()
        } else if matches!(mouse_button, MouseButton::Left) && matches!(input_action, InputAction::Up) {
            return self.mouse_left_up_handler.did_handle()
        } else if matches!(mouse_button, MouseButton::Left) && matches!(input_action, InputAction::UpOrDown) {
            return self.mouse_left_down_handler.did_handle() || self.mouse_left_up_handler.did_handle()
        }
        
        log::engine_warn(format!("InputEventHandlers.does_handle_mouse_event returning false because event was not handled: mouse_button {:?}, input_action {:?}", mouse_button, input_action));
        false
    }

    /// sets handle for all handlers
    pub fn set_handle(&mut self, handle: bool) {
        self.mouse_left_down_handler.set_does_handle(handle);
        self.mouse_left_up_handler.set_does_handle(handle);
        self.mouse_left_down_handler.set_does_handle(handle);
        self.hover_handler.set_does_handle(handle);
        self.scroll_handler.set_does_handle(handle);
        self.mouse_left_drag_handler.set_does_handle(handle);
    }
}

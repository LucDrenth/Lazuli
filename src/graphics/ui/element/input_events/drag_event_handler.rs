use glam::Vec2;

use crate::input::{Input, MouseButton};

use super::EventHandler;

#[derive(Clone, Copy)]
pub struct DragEventHandler {
    event_handler: EventHandler,
    drag_start_position: Vec2,
    did_drag_start: bool,
}

impl DragEventHandler {
    pub fn new(handle: bool, capture: bool) -> Self {
        Self {
            event_handler: EventHandler::new(handle, capture),
            drag_start_position: Vec2::ZERO,
            did_drag_start: false,
        }
    }

    pub fn set_does_handle(&mut self, does_handle: bool) {
        self.event_handler.set_does_handle(does_handle);
    }

    pub fn did_handle(&self) -> bool {
        self.event_handler.did_handle()
    }

    pub fn register(&mut self, input: &Input) -> bool {
        self.event_handler.try_to_handle();

        if self.event_handler.did_handle() {
            self.drag_start_position = input.get_mouse_position()
        }

        self.event_handler.did_capture()
    }

    pub fn reset(&mut self, input: &Input) {
        if input.is_mouse_button_up(MouseButton::Left) {
            self.event_handler.reset();
        }
    }

    pub fn get_drag_start_position(&self) -> Vec2 {
        self.drag_start_position
    }
    pub fn did_drag_start(&self) -> bool {
        self.did_drag_start
    }
}

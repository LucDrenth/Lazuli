use crate::event::{EventReader, self, EventSystem};

use super::Window;

pub struct WindowListeners {
    // TODO put this in to its own struct: WindowListeners
    lock_cursor_listener: EventReader<event::LockCursor>,
    unlock_cursor_listener: EventReader<event::UnlockCursor>,
    confine_cursor_listener: EventReader<event::ConfineCursor>,
    show_cursor_listener: EventReader<event::ShowCursor>,
    hide_cursor_listener: EventReader<event::HideCursor>,
    set_cursor_position_listener: EventReader<event::SetCursorPosition>,
}

impl WindowListeners {
    pub fn new(event_system: &mut EventSystem) -> Self {
        return Self {
            lock_cursor_listener: event_system.register::<event::LockCursor>(),
            unlock_cursor_listener: event_system.register::<event::UnlockCursor>(),
            confine_cursor_listener: event_system.register::<event::ConfineCursor>(),
            show_cursor_listener: event_system.register::<event::ShowCursor>(),
            hide_cursor_listener: event_system.register::<event::HideCursor>(),
            set_cursor_position_listener: event_system.register::<event::SetCursorPosition>(),
        }
    }

    pub fn read(&mut self, glutin_window: &glutin::window::Window) {
        if self.lock_cursor_listener.read().len() > 0 { Window::lock_cursor(glutin_window) }
        if self.unlock_cursor_listener.read().len() > 0 { Window::unlock_cursor(glutin_window) }
        if self.confine_cursor_listener.read().len() > 0 { Window::confine_cursor(glutin_window) }
        if self.hide_cursor_listener.read().len() > 0 { Window::hide_cursor(glutin_window) }
        if self.show_cursor_listener.read().len() > 0 { Window::show_cursor(glutin_window) }
        if let Some(event) = self.set_cursor_position_listener.read().last() {
            Window::set_cursor_position(glutin_window, event.x, event.y);
        }
    }
}

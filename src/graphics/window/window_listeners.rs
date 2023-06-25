use crate::event::{EventReader, self, EventSystem};

pub struct WindowListeners {
    pub lock_cursor_listener: EventReader<event::LockCursor>,
    pub unlock_cursor_listener: EventReader<event::UnlockCursor>,
    pub confine_cursor_listener: EventReader<event::ConfineCursor>,
    pub show_cursor_listener: EventReader<event::ShowCursor>,
    pub hide_cursor_listener: EventReader<event::HideCursor>,
    pub set_cursor_position_listener: EventReader<event::SetCursorPosition>,
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

    // TODO
    // pub fn read(&mut self, window: &dyn Window) {
    //     if self.lock_cursor_listener.read().len() > 0 { window.lock_cursor() }
    //     if self.unlock_cursor_listener.read().len() > 0 { window.unlock_cursor() }
    //     if self.confine_cursor_listener.read().len() > 0 { window.confine_cursor() }
    //     if self.hide_cursor_listener.read().len() > 0 { window.hide_cursor() }
    //     if self.show_cursor_listener.read().len() > 0 { window.show_cursor() }
    //     if let Some(event) = self.set_cursor_position_listener.read().last() {
    //         window.set_cursor_position(event.x, event.y);
    //     }
    // }
}

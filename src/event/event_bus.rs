use super::event::{Event, WindowResizeEvent};

static mut EVENT_BUS: EventBus = EventBus{
    listeners_window_resize: vec![],
};

pub fn send(event: &Event) {
    unsafe { EVENT_BUS.send(event); }
}

struct EventBus {
    listeners_window_resize: Vec<Box<dyn Fn(&WindowResizeEvent) + Send + Sync>>,
}

impl EventBus {
    fn send(&self, event: &Event) {
        match event {
            Event::WindowResize(e) => {
                for listener_function in self.listeners_window_resize.iter() {
                    listener_function(e);
                }
            },
        }
    }
}

// TODO abstractiate this process so we don't have to make a new set of functions for each Event type
impl EventBus {
    fn add_listener_window_resize<F>(&mut self, listener: F) where F: Fn(&WindowResizeEvent) + Send + Sync + 'static {   
        self.listeners_window_resize.push(Box::new(listener));
    }
}
pub fn add_listener_window_resize<F>(listener: F) where F: Fn(&WindowResizeEvent) + Send + Sync + 'static {
        unsafe { EVENT_BUS.add_listener_window_resize(listener) };
}

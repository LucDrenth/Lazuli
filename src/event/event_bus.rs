use super::event::{Event};

static mut EVENT_BUS: EventBus = EventBus{
    listeners: vec![],
};

pub fn add_listener<F>(listener: F)
where F: Fn(&Event) + Send + Sync + 'static, {
        unsafe { EVENT_BUS.add_listener(listener) };
}

pub fn send(event: &Event) {
    unsafe { EVENT_BUS.send(event); }
}


struct EventBus {
    listeners: Vec<Box<dyn Fn(&Event) + Send + Sync>>,
}

impl EventBus {
    fn send(&self, event: &Event) {
        for listener_function in self.listeners.iter() {
            listener_function(event);
        }
    }

    fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }
}

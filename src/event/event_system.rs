use std::any::Any;
use std::collections::HashMap;

use crate::lz_core_info;

type EventCallback = Box<dyn Fn(&dyn Any)>;

struct FooEvent {
    message: String,
}

struct EventDispatcher {
    listeners: HashMap<String, Vec<EventCallback>>,
}

impl EventDispatcher {
    fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn add_listener<T>(&mut self, event_name: &str, callback: impl Fn(&T) + 'static)
    where
        T: 'static,
    {
        let callback_boxed: EventCallback = Box::new(move |event: &dyn Any| {
            if let Some(event) = event.downcast_ref::<T>() {
                callback(event);
            }
        });

        self.listeners
            .entry(event_name.to_string())
            .or_default()
            .push(callback_boxed);
    }

    fn dispatch_event<T>(&self, event_name: &str, event: T)
    where
        T: 'static,
    {
        if let Some(callbacks) = self.listeners.get(event_name) {
            let event_dyn = &event as &dyn Any;
            for callback in callbacks {
                callback(event_dyn);
            }
        }
    }
}




pub fn do_test() {
    let mut event_dispatcher = EventDispatcher::new();

    event_dispatcher.add_listener::<FooEvent>("foo", |event| {
        lz_core_info!("Received FooEvent: {}", event.message);
    });

    event_dispatcher.add_listener::<FooEvent>("foo", |event| {
        lz_core_info!("Another callback for FooEvent: {}", event.message);
    });

    event_dispatcher.add_listener::<FooEvent>("bar", |event| {
        lz_core_info!("Received FooEvent for 'bar': {}", event.message);
    });

    let foo_event = FooEvent {
        message: String::from("Hello, world!"),
    };

    event_dispatcher.dispatch_event("foo", foo_event);
}

use std::any::{Any, TypeId};
use std::collections::HashMap;

type Listener = Box<dyn Fn(&dyn Any)>;

struct FooEvent {
    message: String,
}

struct BarEvent {
    message: String,
}

pub struct EventSystem {
    listeners: HashMap<TypeId, Vec<Listener>>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<T: 'static>(&mut self, callback: impl Fn(&T) + 'static) {
        let boxed_callback: Listener = Box::new(move |event: &dyn Any| {
            if let Some(event) = event.downcast_ref::<T>() {
                callback(event);
            }
        });

        self.listeners
            .entry(TypeId::of::<T>())
            .or_default()
            .push(boxed_callback);
    }

    pub fn send<T: 'static>(&self, event: T) {
        if let Some(callbacks) = self.listeners.get(&TypeId::of::<T>()) {
            let event_dyn = &event as &dyn Any;
            
            for callback in callbacks {
                callback(event_dyn);
            }
        }
    }
}

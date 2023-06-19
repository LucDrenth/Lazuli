use std::{collections::HashMap, any::{TypeId, Any}};

use super::EventReader;

struct Listener<T> {
    sender: bus::Bus<T>,
}

pub struct EventSystem {
    listeners: HashMap<TypeId, Box<dyn Any>>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self { listeners: HashMap::new() }
    }

    pub fn register<T: 'static + std::clone::Clone + std::marker::Sync>(&mut self) -> EventReader<T> {
        let listener = self
            .listeners
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                Box::new(Listener {
                    sender: bus::Bus::<T>::new(256),
                })
            })
            .downcast_mut::<Listener<T>>()
            .unwrap();

        EventReader::<T>::new(listener.sender.add_rx())
    }

    pub fn send<T: Clone + 'static>(&mut self, value: T) {
        if let Some(listener) = self.listeners.get_mut(&TypeId::of::<T>()) {
            let listener = listener.downcast_mut::<Listener<T>>().unwrap();
            listener.sender.broadcast(value);
        }
    }
}

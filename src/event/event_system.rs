use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type Listener = Box<dyn Fn(&dyn Any)>;
pub type ListenerHandle = Rc<RefCell<Listener>>;

pub struct EventSystem {
    listeners: HashMap<TypeId, Vec<ListenerHandle>>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<T: 'static>(&mut self, callback: impl Fn(&T) + 'static) -> ListenerHandle {
        let boxed_callback: Listener = Box::new(move |event: &dyn Any| {
            if let Some(event) = event.downcast_ref::<T>() {
                callback(event);
            }
        });

        let entry: &mut Vec<ListenerHandle> = self.listeners.entry(TypeId::of::<T>()).or_default();
        let listener: ListenerHandle = Rc::new(RefCell::new(boxed_callback));
        entry.push(listener.clone());
        
        return listener;
    }

    pub fn remove_listener<T: 'static>(&mut self, listener: &ListenerHandle) {
        if let Some(callbacks) = self.listeners.get_mut(&TypeId::of::<T>()) {
            callbacks.retain(|stored_listener: &ListenerHandle| !Rc::ptr_eq(listener, stored_listener));
        }
    }

    pub fn send<T: 'static>(&self, event: T) {
        if let Some(callbacks) = self.listeners.get(&TypeId::of::<T>()) {
            let event_dyn = &event as &dyn Any;

            for listener in callbacks {
                listener.borrow()(event_dyn);
            }
        }
    }
}

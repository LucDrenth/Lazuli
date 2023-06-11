use crate::event::EventSystem;

pub trait Scene {
    unsafe fn draw(&self);
    fn update(&mut self, event_system: &mut EventSystem);
}

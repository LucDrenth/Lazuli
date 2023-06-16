use crate::event::EventSystem;

pub trait Scene {
    fn new(event_system: &mut EventSystem) -> Result<Self, String> where Self: Sized;
    unsafe fn draw(&self);
    fn update(&mut self, event_system: &mut EventSystem);
}

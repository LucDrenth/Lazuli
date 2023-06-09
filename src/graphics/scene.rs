use glam::Vec2;

use crate::{event::EventSystem, input::Input};

pub trait Scene {
    fn new(event_system: &mut EventSystem, window_size: Vec2) -> Result<Self, String> where Self: Sized;
    unsafe fn draw(&self);
    fn update(&mut self, event_system: &mut EventSystem, input: &Input);
}

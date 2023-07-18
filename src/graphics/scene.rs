use glam::Vec2;

use crate::{event::EventSystem, input::Input, asset_registry::AssetRegistry};

pub trait Scene {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> where Self: Sized;
    unsafe fn draw(&self, asset_registry: &mut AssetRegistry);
    fn update(&mut self, event_system: &mut EventSystem, input: &Input, asset_registry: &mut AssetRegistry);
}

use glam::Vec2;

use crate::{event::EventSystem, input::Input, asset_manager::AssetManager};

use super::ui::Interface;

pub trait Scene {
    fn new(event_system: &mut EventSystem, window_size: Vec2, pixel_density: f32, asset_manager: &mut dyn AssetManager, interface: &mut Interface) -> Result<Self, String> where Self: Sized;
    unsafe fn draw(&self, asset_manager: &mut dyn AssetManager);
    fn update(&mut self, event_system: &mut EventSystem, input: &Input, asset_manager: &mut dyn AssetManager, interface: &mut Interface);
}

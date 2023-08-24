use lazuli::{graphics::scene::Scene, glam::Vec2, asset_manager::AssetManager, event::EventSystem, input::Input};

pub struct CustomScene {}

impl Scene for CustomScene {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, _pixel_density: f32, _asset_manager: &mut AssetManager) -> Result<Self, String> where Self: Sized {
        Ok(Self {  })
    }

    unsafe fn draw(&self, _asset_manager: &mut AssetManager) {
    }

    fn update(&mut self, _event_system: &mut EventSystem, _input: &Input, _asset_manager: &mut AssetManager) {
    }
}

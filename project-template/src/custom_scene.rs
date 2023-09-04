use lazuli::{graphics::scene::Scene, glam::Vec2, asset_manager::AssetManager, event::EventSystem, input::Input};

pub struct CustomScene {}

impl Scene for CustomScene {
    fn new(_: &mut EventSystem, _: Vec2, _: f32, _: &mut AssetManager, _: &mut Interface) -> Result<Self, String> where Self: Sized {
        Ok(Self {  })
    }

    unsafe fn draw(&self, _: &mut AssetManager) {
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetManager, _: &mut Interface) {
    }
}

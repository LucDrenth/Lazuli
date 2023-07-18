use std::env;

use glam::Vec2;

use crate::asset_registry::AssetRegistry;
use crate::event::EventSystem;
use crate::graphics::scene::Scene;
use crate::graphics::window::{Window, GlutinWindow};
use crate::graphics::renderer::Renderer;
use crate::input::Input;

pub struct App {
    pub event_system: EventSystem,
    window: Box<dyn Window>,
    input: Input,
    pub asset_registry: AssetRegistry,
}

impl App {
    pub fn new() -> Self {
        // TODO since backtrace can be slow in production, we need to disable this in release mode
        env::set_var("RUST_BACKTRACE", "1");

        let mut event_system = EventSystem::new();
        let window = Self::create_window(&mut event_system);
        let input = Input::new();
        let asset_registry = AssetRegistry::new();

        Self { event_system, window, input, asset_registry }
    }

    pub fn window_size(&self) -> Vec2 {
        self.window.get_size()
    }

    pub fn run(self, scene: Box<dyn Scene>) {
        let renderer = Renderer::new(scene).expect("Could not create renderer");
        self.window.run(renderer, self.event_system, self.input, self.asset_registry);
    }

    fn create_window(event_system: &mut EventSystem) -> Box<dyn Window> {
        let window = GlutinWindow::new(String::from("Lazuli"), event_system);
        Box::new(window)
    }
}

use std::env;

use crate::asset_manager::AssetManager;
use crate::event::EventSystem;
use crate::graphics::scene::Scene;
use crate::graphics::window::{Window, GlutinWindow, WindowBuilder};
use crate::graphics::renderer::Renderer;
use crate::input::Input;

pub struct App {
    pub event_system: EventSystem,
    window: Box<dyn Window>,
    input: Input,
    pub asset_manager: AssetManager,
}

impl App {
    pub fn new<T: Scene + 'static>(window_builder: WindowBuilder) {
        // TODO since backtrace can be slow, we need to disable this in release mode
        env::set_var("RUST_BACKTRACE", "1");

        let mut event_system = EventSystem::new();
        let window = Self::create_window(window_builder, &mut event_system);
        let input = Input::new();
        let asset_manager = AssetManager::new();

        let app = Self { event_system, window, input, asset_manager };
        app.run::<T>();
    }

    fn create_window(window_builder: WindowBuilder, event_system: &mut EventSystem) -> Box<dyn Window> {
        let window = GlutinWindow::new(window_builder, event_system);
        Box::new(window)
    }

    fn run<T: Scene + 'static>(mut self) {
        let scene = T::new(
            &mut self.event_system, 
            self.window.get_size(), 
            self.window.get_pixel_density() as f32, 
            &mut self.asset_manager
        ).expect("App failed to create initial scene");
        
        let renderer = Renderer::new(Box::new(scene)).expect("App failed to create renderer");
        self.window.run(renderer, self.event_system, self.input, self.asset_manager);
    }
}

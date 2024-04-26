use std::env;

use crate::asset_manager::{AssetManager, GlAssetManager};
use crate::event::EventSystem;
use crate::graphics::scene::Scene;
use crate::graphics::ui::Interface;
use crate::graphics::window::{Window, WindowBuilder};
use crate::graphics::renderer::Renderer;
use crate::input::Input;

pub struct App {
    pub event_system: EventSystem,
    window: Box<dyn Window>,
    input: Input,
    pub asset_manager: Box<dyn AssetManager>,
    interface: Interface,
}

impl App {
    pub fn new<T: Scene + 'static>(window_builder: WindowBuilder) {
        // TODO since backtrace can be slow, we need to disable this in release mode
        env::set_var("RUST_BACKTRACE", "1");

        let mut event_system = EventSystem::new();
        let window = window_builder.build(&mut event_system);
        let input = Input::new();
        let asset_manager = Box::new(GlAssetManager::new());
        let interface = Interface::new(&mut event_system, window.get_size(), window.get_pixel_density() as f32);

        let app = Self { event_system, window, input, asset_manager, interface };
        app.run::<T>();
    }

    fn run<T: Scene + 'static>(mut self) {
        let scene = T::new(
            &mut self.event_system, 
            self.window.get_size(), 
            self.window.get_pixel_density() as f32, 
            &mut *self.asset_manager,
            &mut self.interface
        ).expect("App failed to create initial scene");
        
        let renderer = Renderer::new(Box::new(scene)).expect("App failed to create renderer");
        self.window.run(renderer, self.event_system, self.input, self.asset_manager, self.interface);
    }
}

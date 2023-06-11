use std::env;

use crate::event::EventSystem;
use crate::graphics::scene::Scene;
use crate::graphics::window::Window;
use crate::graphics::renderer::Renderer;

pub struct App {
    pub event_system: EventSystem,
    window: Window,
}

impl App {
    pub fn new() -> Self {
        // TODO since backtrace can be slow in production, we need to disable this in release mode
        env::set_var("RUST_BACKTRACE", "1");

        let event_system = EventSystem::new();
        let window = Window::new(String::from("Lazuli"));

        Self { event_system, window }
    }

    pub fn run(self, scene: Box<dyn Scene>) {
        let renderer = Renderer::new(scene).expect("Could not create renderer");
        self.window.run(renderer, self.event_system);
    }
}

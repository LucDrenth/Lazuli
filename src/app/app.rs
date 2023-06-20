use std::env;

use crate::event::EventSystem;
use crate::graphics::scene::Scene;
use crate::graphics::window::Window;
use crate::graphics::renderer::Renderer;
use crate::input::Input;

pub struct App {
    pub event_system: EventSystem,
    window: Window,
    input: Input,
}

impl App {
    pub fn new() -> Self {
        // TODO since backtrace can be slow in production, we need to disable this in release mode
        env::set_var("RUST_BACKTRACE", "1");

        let event_system = EventSystem::new();
        let window = Window::new(String::from("Lazuli"));
        let input = Input::new();

        Self { event_system, window, input }
    }

    pub fn run(self, scene: Box<dyn Scene>) {
        let renderer = Renderer::new(scene).expect("Could not create renderer");
        self.window.run(renderer, self.event_system, self.input);
    }
}

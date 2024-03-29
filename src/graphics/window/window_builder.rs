use crate::event::EventSystem;

use super::{WindowApi, Window};

pub struct WindowBuilder {
    pub name: String,
    pub size: WindowSize,
    pub resizable: bool,
    pub window_api: WindowApi,
}

pub enum WindowSize {
    FullScreen, // As big as the monitor, removing OS HUD
    Maximized, // As big as the monitor, keeping the OS HUD
    Pixels(u32, u32)
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self { 
            name: "Lazuli project".to_string(),
            size: WindowSize::Pixels(800, 600),
            resizable: true,
            window_api: WindowApi::Glutin,
        }
    }

    pub fn build(&self, event_system: &mut EventSystem) -> Box<dyn Window> {
        self.window_api.build(&self, event_system)
    }

    // setters start here

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = WindowSize::Pixels(width, height);
        self
    }

    pub fn with_full_screen(mut self) -> Self {
        self.size = WindowSize::FullScreen;
        self
    }

    pub fn with_maximized(mut self) -> Self {
        self.size = WindowSize::Maximized;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_window_api(mut self, window_api: WindowApi) -> Self {
        self.window_api = window_api;
        self
    }
}

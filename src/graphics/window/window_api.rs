use crate::event::EventSystem;

use super::{Window, GlutinWindow, WindowBuilder};

pub enum WindowApi {
    Glutin,
}

impl WindowApi {
    pub fn build(&self, window_builder: &WindowBuilder, event_system: &mut EventSystem) -> Box<dyn Window> {
        match self {
            WindowApi::Glutin => {
                let window = GlutinWindow::new(window_builder, event_system);
                Box::new(window)
            },
        }
    }
}

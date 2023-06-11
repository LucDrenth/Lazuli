use crate::{graphics::scene::Scene, lz_core_info, event::{WindowResizeEvent, EventSystem}};

pub struct EventBusScene {}

impl EventBusScene {
    pub fn new(event_system: &mut EventSystem) -> Result<Self, String> {
        event_system.add_listener::<WindowResizeEvent>(window_resize_listener);
        Ok(Self{})
    }
}

impl Scene for EventBusScene {
    fn update(&mut self) {}

    unsafe fn draw(&self) {}
}

fn window_resize_listener(event: &WindowResizeEvent) {
    lz_core_info!("Window resize event: {} / {}", event.width, event.height);
}

use crate::{graphics::scene::Scene, event::{event_bus, event::WindowResizeEvent}, lz_core_info};

pub struct EventBusScene {}

impl EventBusScene {
    pub fn new() -> Result<Self, String> {
        event_bus::add_listener_window_resize(my_test);

        Ok(Self{})
    }
}

impl Scene for EventBusScene {
    fn update(&mut self) {}

    unsafe fn draw(&self) {}
}

fn my_test(e: &WindowResizeEvent) {
    lz_core_info!("Window resize event: {} / {}", e.width, e.height);
}

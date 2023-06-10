use crate::{graphics::scene::Scene, event::{event_bus, Event}};

pub struct EventBusScene {}

impl EventBusScene {
    pub fn new() -> Result<Self, String> {
        event_bus::add_listener(my_test);

        Ok(Self{})
    }
}

impl Scene for EventBusScene {
    fn update(&mut self) {}

    unsafe fn draw(&self) {}
}

fn my_test(event: &Event) {
    match event {
        Event::WindowResize(e) => {
            println!("Window resize event: {} / {}", e.width, e.height);
        },
    }
}

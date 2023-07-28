use glam::Vec2;

use crate::{graphics::scene::Scene, event::{WindowResizeEvent, EventSystem, EventReader}, input::Input, asset_registry::AssetRegistry, log};

pub struct HelloEventSystem {
    window_resize_listener1: EventReader<WindowResizeEvent>,
    window_resize_listener2: EventReader<WindowResizeEvent>,
    nr_updates: u32,
}

impl Scene for HelloEventSystem {
    fn new(event_system: &mut EventSystem, _window_size: Vec2, _: &mut AssetRegistry) -> Result<Self, String> {
        let listener1 = event_system.register::<WindowResizeEvent>();
        let listener2 = event_system.register::<WindowResizeEvent>();
        
        Ok(Self{
            window_resize_listener1: listener1,
            window_resize_listener2: listener2,
            nr_updates: 0,
        })
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetRegistry) {
        for event in self.window_resize_listener1.read().iter() {
            log::engine_info(format!("(1) Window resize event: {} / {}", event.width, event.height));
        }

        for event in self.window_resize_listener2.read().iter() {
            log::engine_info(format!("(2) Window resize event: {} / {}", event.width, event.height));
        }

        self.nr_updates += 1;

        if self.nr_updates == 300 { // 5 seconds
            log::engine_info(format!("removing WindowResizeEvent listener 1"));
            self.window_resize_listener1.close();
        }
    }

    unsafe fn draw(&self, _: &mut AssetRegistry) {}
}

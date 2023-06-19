use crate::{graphics::scene::Scene, lz_core_info, event::{WindowResizeEvent, EventSystem, EventReader}};

pub struct HelloEventSystem {
    window_resize_listener1: EventReader<WindowResizeEvent>,
    window_resize_listener2: EventReader<WindowResizeEvent>,
    nr_updates: u32,
}

impl Scene for HelloEventSystem {
    fn new(event_system: &mut EventSystem) -> Result<Self, String> {
        let listener1 = event_system.register::<WindowResizeEvent>();
        let listener2 = event_system.register::<WindowResizeEvent>();
        
        Ok(Self{
            window_resize_listener1: listener1,
            window_resize_listener2: listener2,
            nr_updates: 0,
        })
    }

    fn update(&mut self, _: &mut EventSystem) {
        for event in self.window_resize_listener1.read().iter() {
            lz_core_info!("(1) Window resize event: {} / {}", event.width, event.height);
        }

        for event in self.window_resize_listener2.read().iter() {
            lz_core_info!("(2) Window resize event: {} / {}", event.width, event.height);
        }

        self.nr_updates += 1;

        if self.nr_updates == 300 { // 5 seconds
            lz_core_info!("removing WindowResizeEvent listener 1");
            self.window_resize_listener1.close();
        }
    }

    unsafe fn draw(&self) {}
}

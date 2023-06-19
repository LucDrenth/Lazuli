use crate::{graphics::scene::Scene, lz_core_info, event::{WindowResizeEvent, EventSystem, EventReader}};

pub struct HelloEventSystem {
    window_resize_listener: EventReader<WindowResizeEvent>,
    nr_updates: u32,
}

impl Scene for HelloEventSystem {
    fn new(event_system: &mut EventSystem) -> Result<Self, String> {
        let listener = event_system.register::<WindowResizeEvent>();        
        
        Ok(Self{
            window_resize_listener: listener,
            nr_updates: 0,
        })
    }

    fn update(&mut self, event_system: &mut EventSystem) {
        for event in self.window_resize_listener.read().iter() {
            lz_core_info!("Window resize event: {} / {}", event.width, event.height);
        }

        self.nr_updates += 1;

        if self.nr_updates == 300 { // 5 seconds
            lz_core_info!("removing WindowResizeEvent listener");
            // event_system.remove_listener::<WindowResizeEvent>(&self.window_resize_listener_handle);
        }
    }

    unsafe fn draw(&self) {}
}

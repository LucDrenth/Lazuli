use crate::{graphics::scene::Scene, lz_core_info, event::{WindowResizeEvent, EventSystem, ListenerHandle}};

pub struct EventBusScene {
    window_resize_listener_handle: ListenerHandle,
    nr_updates: u32,
}

impl EventBusScene {
    pub fn new(event_system: &mut EventSystem) -> Result<Self, String> {
        let handle: ListenerHandle = event_system.add_listener::<WindowResizeEvent>(window_resize_listener);        
        
        Ok(Self{
            window_resize_listener_handle: handle,
            nr_updates: 0,
        })
    }
}

impl Scene for EventBusScene {
    fn update(&mut self, event_system: &mut EventSystem) {
        self.nr_updates += 1;

        if self.nr_updates == 300 { // 5 seconds
            lz_core_info!("removing WindowResizeEvent listener");
            event_system.remove_listener::<WindowResizeEvent>(&self.window_resize_listener_handle);
        }
    }

    unsafe fn draw(&self) {}
}

fn window_resize_listener(event: &WindowResizeEvent) {
    lz_core_info!("Window resize event: {} / {}", event.width, event.height);
}

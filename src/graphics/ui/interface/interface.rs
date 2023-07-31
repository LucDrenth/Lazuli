use glam::Vec2;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, asset_registry::AssetRegistry, input::Input};

use super::ElementRegistry;

pub struct Interface {
    element_registry: ElementRegistry,
    window_resize_listener: EventReader<WindowResizeEvent>,
    size: Vec2,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem, window_size: Vec2) -> Self {
        Self {
            element_registry: ElementRegistry::new(window_size),
            window_resize_listener: event_system.register::<WindowResizeEvent>(),
            size: window_size,
        }
    }

    pub fn update(&mut self, asset_registry: &mut AssetRegistry, input: &Input) {
        self.element_registry.update(asset_registry, input);
        
        self.window_resize_listener.read().last().map(|e| {
            let window_size = Vec2::new(e.width as f32, e.height as f32);
            self.element_registry.handle_window_resize(window_size, asset_registry);
        });
    }

    pub fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.element_registry.draw(asset_registry);
    }

    pub fn mut_element_registry(&mut self) -> &mut ElementRegistry { &mut self.element_registry }
}

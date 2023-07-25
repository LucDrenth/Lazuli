use glam::Vec2;

use crate::{event::EventSystem, graphics::renderer::Renderer, input::Input, asset_registry::AssetRegistry};

pub trait Window {
    fn run(self: Box<Self>, renderer: Renderer, event_system: EventSystem, lz_input: Input, asset_registry: AssetRegistry);

    /// in logical units (physical size / DPI)
    fn get_size(&self) -> Vec2;
    
    // TODO
    // fn lock_cursor(&self);
    // fn unlock_cursor(&self);
    // fn confine_cursor(&self);
    // fn hide_cursor(&self);
    // fn show_cursor(&self);
    // fn set_cursor_position(&self, x: f32, y: f32);
}

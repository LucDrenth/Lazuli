use glam::Vec2;

use crate::{event::EventSystem, graphics::{renderer::Renderer, ui::Interface}, input::Input, asset_manager::AssetManager};

pub trait Window {
    fn run(self: Box<Self>, renderer: Renderer, event_system: EventSystem, lz_input: Input, asset_manager: Box<dyn AssetManager>, interface: Interface);

    /// in logical units (physical size / DPI)
    fn get_size(&self) -> Vec2;

    /// total size of decorators (like the top bar that has the window title, close button etc.)
    fn frame_size(&self) -> Vec2;

    /// dpi
    fn get_pixel_density(&self) -> f64;
    
    // TODO
    // fn lock_cursor(&self);
    // fn unlock_cursor(&self);
    // fn confine_cursor(&self);
    // fn hide_cursor(&self);
    // fn show_cursor(&self);
    // fn set_cursor_position(&self, x: f32, y: f32);
}

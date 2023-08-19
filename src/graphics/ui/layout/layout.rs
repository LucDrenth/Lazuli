use crate::{graphics::ui::Interface, input::Input};

/// Widgets in a layout should get a higher z_index than the background
pub const LAYOUT_ELEMENT_EXTRA_Z_INDEX: f32  = 0.1;

pub trait Layout {
    fn add_widget(&mut self, widget_id: u32, interface: &mut Interface);
    fn update(&mut self, interface: &mut Interface, input: &Input);
}

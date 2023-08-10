use crate::graphics::ui::ElementRegistry;

pub trait UiWidget {
    fn show(&self, element_registry: &mut ElementRegistry);
    fn hide(&self, element_registry: &mut ElementRegistry);
    fn anchor_element_id(&self) -> u32;
    fn z_index(&self) -> f32;
}

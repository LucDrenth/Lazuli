use crate::asset_registry::AssetRegistry;

pub trait UiElement {
    fn material_id(&self) -> u32;
    fn draw(&self, asset_registry: &mut AssetRegistry);
    fn get_z_index(&self) -> f32;
}

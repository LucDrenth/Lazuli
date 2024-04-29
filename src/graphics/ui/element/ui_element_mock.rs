use glam::Vec2;

use crate::{asset_manager::AssetManager, graphics::{material::Material, shader::CustomShaderValues, Color}, ResourceId};

use super::{ui_element::UiElement, world_element_data::WorldElementData};

pub struct UiElementMock {
    pub material_id: ResourceId<Material>,
    pub world_element_data: WorldElementData,
    pub custom_shader_values: CustomShaderValues,
}


impl Default for UiElementMock {
    fn default() -> Self {
        Self { 
            material_id: ResourceId::new(0), 
            world_element_data: WorldElementData::new_mock(),
            custom_shader_values: Default::default() 
        }
    }
}

impl UiElement for UiElementMock {
    fn material_id(&self) -> &crate::ResourceId<Material> {
        &self.material_id
    }

    fn draw(&self, _asset_manager: &mut dyn AssetManager, _window_size: &Vec2, _pixel_density: f32) {}

    fn type_name(&self) -> &str {
        "mock"
    }

    fn handle_window_resize(&mut self, _new_window_size: &Vec2) {}

    fn world_data(&self) -> &WorldElementData {
        &self.world_element_data
    }

    fn mut_world_data(&mut self) -> &mut super::world_element_data::WorldElementData {
        &mut self.world_element_data
    }

    fn mut_custom_shader_values(&mut self) -> &mut CustomShaderValues {
        &mut self.custom_shader_values
    }

    fn set_color(&mut self, _color: Color) {}
}

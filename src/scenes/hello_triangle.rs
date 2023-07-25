use glam::Vec2;

use crate::{graphics::{scene::Scene, Triangle, shader::{PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG, ShaderBuilder, ShaderProgram}, Shape}, event::EventSystem, input::Input, asset_registry::{AssetRegistry, AssetId}};

pub struct HelloTriangle {
    shader_id: AssetId<ShaderProgram>,
    triangle: Triangle,
}

impl Scene for HelloTriangle {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> {
        let shader_id = asset_registry.load_shader(ShaderBuilder::new()
            .with_vertex_shader_path(PATH_MOVING_TRIANGLE_VERT.to_string())
            .with_fragment_shader_path(PATH_MOVING_TRIANGLE_FRAG.to_string())
        )?;

        let triangle = Triangle::new(asset_registry.get_shader_by_id(&shader_id).unwrap());

        let result = Self { 
            shader_id,
            triangle,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetRegistry) {}

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.triangle.draw(asset_registry.get_shader_by_id(&self.shader_id).unwrap())
    }
}

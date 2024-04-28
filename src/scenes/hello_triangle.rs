use glam::Vec2;

use crate::{asset_manager::AssetManager, event::EventSystem, graphics::{scene::Scene, shader::{GlShaderBuilder, ShaderProgram, PATH_MOVING_TRIANGLE_FRAG, PATH_MOVING_TRIANGLE_VERT}, ui::Interface, Shape, Triangle}, input::Input, ResourceId};

pub struct HelloTriangle {
    shader_id: ResourceId<Box<dyn ShaderProgram>>,
    triangle: Triangle,
}

impl Scene for HelloTriangle {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, _pixel_density: f32, asset_manager: &mut dyn AssetManager, _: &mut Interface) -> Result<Self, String> {
        let shader_id = asset_manager.load_shader(
            Box::new(GlShaderBuilder::new(PATH_MOVING_TRIANGLE_VERT, PATH_MOVING_TRIANGLE_FRAG))
        )?;

        let triangle = Triangle::new(asset_manager.get_shader_by_id(&shader_id).unwrap());

        let result = Self { 
            shader_id,
            triangle,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut dyn AssetManager, _: &mut Interface) {}

    unsafe fn draw(&self, asset_manager: &mut dyn AssetManager) {
        self.triangle.draw(asset_manager.get_shader_by_id(&self.shader_id).unwrap())
    }
}

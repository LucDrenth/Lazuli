use glam::Vec2;

use crate::{graphics::{scene::Scene, shader::{PATH_TEXTURED_VERT, PATH_TEXTURED_FRAG, ShaderBuilder}, Rectangle, Shape, material::Material}, event::EventSystem, input::Input, asset_registry::{AssetRegistry, AssetId}};

pub struct HelloTexture {
    material_id: AssetId<Material>,
    shape: Rectangle,
}

impl Scene for HelloTexture {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> {
        let shader_id = asset_registry.load_shader(ShaderBuilder::new()
            .with_vertex_shader_path(PATH_TEXTURED_VERT.to_string())
            .with_fragment_shader_path(PATH_TEXTURED_FRAG.to_string())
        )?;
        let material_id = asset_registry.load_material(&shader_id)?;
        let texture_id = asset_registry.load_texture("./assets/images/pattern.png".to_string())?;
        asset_registry.add_material_texture(&material_id, &texture_id);

        let shape = Rectangle::new_textured(asset_registry.get_shader_by_id(&shader_id).unwrap());

        let result = Self { 
            material_id,
            shape,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetRegistry) {}

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.shape.draw(asset_registry.get_material_shader(&self.material_id).unwrap());
    }
}

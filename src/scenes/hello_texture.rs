use glam::Vec2;

use crate::{asset_manager::AssetManager, event::EventSystem, graphics::{material::Material, scene::Scene, shader::{ShaderBuilder, PATH_TEXTURED_FRAG, PATH_TEXTURED_VERT}, ui::Interface, Rectangle, Shape}, input::Input, ResourceId};

pub struct HelloTexture {
    material_id: ResourceId<Material>,
    shape: Rectangle,
}

impl Scene for HelloTexture {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, _pixel_density: f32, asset_manager: &mut dyn AssetManager, _: &mut Interface) -> Result<Self, String> {
        let shader_id = asset_manager.load_shader(
            ShaderBuilder::new(PATH_TEXTURED_VERT, PATH_TEXTURED_FRAG)
        )?;
        let material_id = asset_manager.load_material(&shader_id)?;
        let texture_id = asset_manager.load_texture(&format!("./assets/images/pattern.png"))?;
        asset_manager.add_material_texture(&material_id, &texture_id);

        let shape = Rectangle::new_textured(asset_manager.get_shader_by_id(&shader_id).unwrap());

        let result = Self { 
            material_id,
            shape,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut dyn AssetManager, _: &mut Interface) {}

    unsafe fn draw(&self, asset_manager: &mut dyn AssetManager) {
        self.shape.draw(asset_manager.get_material_shader(&self.material_id).unwrap());
    }
}

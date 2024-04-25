use crate::{asset_manager::AssetManagerTrait, graphics::{font, material::Material, shader::{ShaderBuilder, ShaderProgram}, texture::{Texture, TextureImage}}, ResourceId};

pub struct MockAssetManager {}

impl AssetManagerTrait for MockAssetManager {
    fn load_texture(&mut self, path: impl Into<String>) -> Result<ResourceId<Box<dyn Texture>>, String> {
        todo!()
    }

    fn load_texture_from_image(&mut self, texture_image: &dyn TextureImage) -> Result<ResourceId<Box<dyn Texture>>, String> {
        todo!()
    }

    fn get_texture_by_id(&mut self, id: &ResourceId<Box<dyn Texture>>) -> Option<&Box<dyn Texture>> {
        todo!()
    }

    fn load_font(&mut self, bitmap_builder: impl font::BitmapBuilder, shader_builder: Option<ShaderBuilder>) -> Result<ResourceId<font::Font>, String> {
        todo!()
    }

    fn get_font_by_id(&mut self, id: &ResourceId<font::Font>) -> Option<&font::Font> {
        todo!()
    }

    fn load_shader(&mut self, shader_builder: ShaderBuilder) -> Result<ResourceId<ShaderProgram>, String> {
        todo!()
    }

    fn get_shader_by_id(&mut self, id: &ResourceId<ShaderProgram>) -> Option<&ShaderProgram> {
        todo!()
    }

    fn load_material(&mut self, shader_id: &ResourceId<ShaderProgram>) -> Result<ResourceId<Material>, String> {
        todo!()
    }

    fn get_material_by_id(&mut self, id: &ResourceId<Material>) -> Option<&mut Material> {
        todo!()
    }

    fn add_material_texture(&mut self, material_id: &ResourceId<Material>, texture_id: &ResourceId<Box<dyn Texture>>) {
        todo!()
    }

    fn activate_material(&mut self, material_id: &ResourceId<Material>) {
        todo!()
    }

    fn get_material_shader(&mut self, material_id: &ResourceId<Material>) -> Option<&ShaderProgram> {
        todo!()
    }
}

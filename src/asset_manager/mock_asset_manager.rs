use glam::Vec2;

use crate::{asset_manager::{AssetCollection, AssetManager}, graphics::{font, material::Material, shader::{ShaderBuilder, ShaderProgram}, texture::{mock_texture::MockTexture, Texture, TextureImage}}, ResourceId};

pub struct MockAssetManager {
    textures: AssetCollection<Box<dyn Texture>, u32>,
    builder_hash_counter: u32,
}

impl MockAssetManager {
    pub fn new() -> Self {
        Self { textures: AssetCollection::new(), builder_hash_counter: 0 }
    }
}

impl AssetManager for MockAssetManager {
    fn load_texture(&mut self, _path: &String) -> Result<ResourceId<Box<dyn Texture>>, String> {
        let mock_texture = MockTexture {
            size: Vec2{x: 720.0, y: 1280.0}
        };
        let builder_hash  = self.new_hash();

        self.textures.add(Box::new(mock_texture), builder_hash)
    }

    fn load_texture_from_image(&mut self, _texture_image: &dyn TextureImage) -> Result<ResourceId<Box<dyn Texture>>, String> {
        let mock_texture = MockTexture {
            size: Vec2{x: 720.0, y: 1280.0}
        };
        let builder_hash  = self.new_hash();

        self.textures.add(Box::new(mock_texture), builder_hash)
    }

    fn get_texture_by_id(&mut self, id: &ResourceId<Box<dyn Texture>>) -> Option<&Box<dyn Texture>> {
        self.textures.get_asset_by_id(id)
    }

    fn load_font(&mut self, _bitmap_builder: &dyn font::BitmapBuilder, _shader_builder: Option<ShaderBuilder>) -> Result<ResourceId<font::Font>, String> {
        todo!()
    }

    fn get_font_by_id(&mut self, _id: &ResourceId<font::Font>) -> Option<&font::Font> {
        todo!()
    }

    fn load_shader(&mut self, _shader_builder: ShaderBuilder) -> Result<ResourceId<ShaderProgram>, String> {
        todo!()
    }

    fn get_shader_by_id(&mut self, _id: &ResourceId<ShaderProgram>) -> Option<&ShaderProgram> {
        todo!()
    }

    fn load_material(&mut self, _shader_id: &ResourceId<ShaderProgram>) -> Result<ResourceId<Material>, String> {
        todo!()
    }

    fn get_material_by_id(&mut self, _id: &ResourceId<Material>) -> Option<&mut Material> {
        todo!()
    }

    fn add_material_texture(&mut self, _material_id: &ResourceId<Material>, _texture_id: &ResourceId<Box<dyn Texture>>) {
        todo!()
    }

    fn activate_material(&mut self, _material_id: &ResourceId<Material>) {
        todo!()
    }

    fn get_material_shader(&mut self, _material_id: &ResourceId<Material>) -> Option<&ShaderProgram> {
        todo!()
    }
}

impl MockAssetManager {
    fn new_hash(&mut self) -> u32 {
        self.builder_hash_counter += 1;
        self.builder_hash_counter
    }
}

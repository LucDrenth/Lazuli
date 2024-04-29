use std::collections::HashMap;

use glam::Vec2;

use crate::{asset_manager::{AssetCollection, AssetManager}, graphics::{font::{self, bitmap_mock::MockBitmap, font_mock::MockFont, Font}, material::Material, shader::{shader_builder_mock::MockShaderBuilder, ShaderBuilder, ShaderProgram}, texture::{texture_mock::MockTexture, Texture, TextureImage}}, ResourceId};

pub struct MockAssetManager {
    textures: AssetCollection<Box<dyn Texture>, u32>,
    fonts: AssetCollection<Box<dyn Font>, u32>,
    shaders: AssetCollection<Box<dyn ShaderProgram>, u32>,
    materials: AssetCollection<Material, bool>,
    builder_hash_counter: u32,
}

impl MockAssetManager {
    pub fn new() -> Self {
        Self {
            textures: AssetCollection::new(),
            fonts: AssetCollection::new(),
            shaders: AssetCollection::new(),
            materials: AssetCollection::new(),
            builder_hash_counter: 0,
        }
    }
}

impl AssetManager for MockAssetManager {
    fn load_texture(&mut self, _path: &String) -> Result<ResourceId<Box<dyn Texture>>, String> {
        let texture = MockTexture {
            size: Vec2{x: 720.0, y: 1280.0}
        };
        let builder_hash  = self.new_hash();

        self.textures.add(Box::new(texture), builder_hash)
    }

    fn load_texture_from_image(&mut self, _texture_image: &dyn TextureImage) -> Result<ResourceId<Box<dyn Texture>>, String> {
        let texture = MockTexture {
            size: Vec2{x: 720.0, y: 1280.0}
        };
        let builder_hash  = self.new_hash();

        self.textures.add(Box::new(texture), builder_hash)
    }

    fn get_texture_by_id(&mut self, id: &ResourceId<Box<dyn Texture>>) -> Option<&Box<dyn Texture>> {
        self.textures.get_asset_by_id(id)
    }

    fn load_font(&mut self, _bitmap_builder: &dyn font::BitmapBuilder, _shader_builder: Option<Box<dyn ShaderBuilder>>) -> Result<ResourceId<Box<dyn Font>>, String> {
        let atlas = MockBitmap {
            characters: HashMap::new(),
            line_height: 5.0,
            spread: 8,
            json_cache: String::new(),
        };

        let shader_builder = MockShaderBuilder{
            hash: 0,
        };
        let shader_id = self.load_shader(Box::new(shader_builder))?;
        let material_id = self.load_material(&shader_id)?;

        let font = MockFont {
            atlas: Box::new(atlas),
            line_height: 5.0,
            space_size: 3.0,
            bitmap_spread: 8,
            material_id,
        };

        let hash = self.new_hash();
        self.fonts.add(Box::new(font), hash)
    }

    fn get_font_by_id(&mut self, id: &ResourceId<Box<dyn Font>>) -> Option<&Box<dyn Font>> {
        self.fonts.get_asset_by_id(id)
    }

    fn load_shader(&mut self, shader_builder: Box<dyn ShaderBuilder>) -> Result<ResourceId<Box<dyn ShaderProgram>>, String> {
        let shader = shader_builder.build()?;
        let hash = self.new_hash();
        self.shaders.add(shader, hash)
    }

    fn get_shader_by_id(&mut self, id: &ResourceId<Box<dyn ShaderProgram>>) -> Option<&Box<dyn ShaderProgram>> {
        self.shaders.get_asset_by_id(id)
    }

    fn load_material(&mut self, shader_id: &ResourceId<Box< dyn ShaderProgram>>) -> Result<ResourceId<Material>, String> {
        let material = Material::new(shader_id.duplicate());
        self.materials.add(material, false)
    }

    fn get_material_by_id(&mut self, id: &ResourceId<Material>) -> Option<&mut Material> {
        self.materials.get_mut_asset_by_id(id)
    }

    fn add_material_texture(&mut self, material_id: &ResourceId<Material>, texture_id: &ResourceId<Box<dyn Texture>>) -> Result<(), String> {
        match self.get_material_by_id(material_id) {
            Some(material) => {
                material.push_texture_id(texture_id.duplicate());
                Ok(())
            },
            None => Err(format!("Material not found")),
            
        }
    }

    fn activate_material(&mut self, material_id: &ResourceId<Material>) -> Result<(), String> {
        match self.get_material_by_id(material_id) {
            Some(_material) => Ok(()),
            None => Err(format!("Material not found")),
        }
    }

    fn get_material_shader(&mut self, material_id: &ResourceId<Material>) -> Option<&Box<dyn ShaderProgram>> {
        let shader_id;

        match self.get_material_by_id(material_id) {
            Some(material) => shader_id = material.shader_id.duplicate(),
            None => return None,
        }

        self.get_shader_by_id(&shader_id)
    }
}

impl MockAssetManager {
    fn new_hash(&mut self) -> u32 {
        self.builder_hash_counter += 1;
        self.builder_hash_counter
    }
}

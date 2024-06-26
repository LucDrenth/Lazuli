use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use crate::{graphics::{font::{BitmapBuilder, Font, GlFont}, material::Material, shader::{ShaderBuilder, ShaderProgram, UniformValue}, texture::{GlTexture, Texture, TextureImage}}, ResourceId};

use super::asset_collection::AssetCollection;

pub trait AssetManager {
    fn load_texture(&mut self, path: &String) -> Result<ResourceId<Box<dyn Texture>>, String>;
    fn load_texture_from_image(&mut self, texture_image: &dyn TextureImage) -> Result<ResourceId<Box<dyn Texture>>, String>;
    fn get_texture_by_id(&mut self, id: &ResourceId<Box<dyn Texture>>) -> Option<&Box<dyn Texture>>;
    fn load_font(&mut self, bitmap_builder: &dyn BitmapBuilder, shader_builder: Option<Box<dyn ShaderBuilder>>) -> Result<ResourceId<Box<dyn Font>>, String>;
    fn get_font_by_id(&mut self, id: &ResourceId<Box<dyn Font>>) -> Option<&Box<dyn Font>>;
    fn load_shader(&mut self, shader_builder: Box<dyn ShaderBuilder>) -> Result<ResourceId<Box<dyn ShaderProgram>>, String>;
    fn get_shader_by_id(&mut self, id: &ResourceId<Box<dyn ShaderProgram>>) -> Option<&Box<dyn ShaderProgram>>;
    fn load_material(&mut self, shader_id: &ResourceId<Box<dyn ShaderProgram>>) -> Result<ResourceId<Material>, String>;
    fn get_material_by_id(&mut self, id: &ResourceId<Material>) -> Option<&mut Material>;
    fn add_material_texture(&mut self, material_id: &ResourceId<Material>, texture_id: &ResourceId<Box<dyn Texture>>) -> Result<(), String>;
    fn activate_material(&mut self, material_id: &ResourceId<Material>) -> Result<(), String>;
    fn get_material_shader(&mut self, material_id: &ResourceId<Material>) -> Option<&Box<dyn ShaderProgram>>;
}

pub struct GlAssetManager {
    textures: AssetCollection<Box<dyn Texture>, Option<String>>,
    fonts: AssetCollection<Box<dyn Font>, u64>,
    shaders: AssetCollection<Box< dyn ShaderProgram>, u64>,
    materials: AssetCollection<Material, bool>,
}

impl GlAssetManager {
    pub fn new() -> Self {
        Self {
            textures: AssetCollection::new(),
            fonts: AssetCollection::new(),
            shaders: AssetCollection::new(),
            materials: AssetCollection::new(),
        }
    }
}

impl AssetManager for GlAssetManager {
    fn load_texture(&mut self, path: &String) -> Result<ResourceId<Box<dyn Texture>>, String> {
        let some_path = Some(path.clone());

        match self.textures.get_by_builder_hash(&some_path) {
            Some(existing) => {
                return Ok(existing)
            },
            None => (),
        }

        match GlTexture::new_from_path(path) {
            Ok(texture) => self.textures.add(Box::new(texture), some_path),
            Err(err) => Err(err),
        }
    }

    fn load_texture_from_image(&mut self, texture_image: &dyn TextureImage) -> Result<ResourceId<Box<dyn Texture>>, String> {
        match GlTexture::new_from_image(texture_image) {
            Ok(texture) => self.textures.add(Box::new(texture), None),
            Err(err) => Err(err),
        }
    }

    fn get_texture_by_id(&mut self, id: &ResourceId<Box<dyn Texture>>) -> Option<&Box<dyn Texture>> {
       self.textures.get_asset_by_id(id)
    }

    fn load_font(&mut self, bitmap_builder: &dyn BitmapBuilder, shader_builder: Option<Box<dyn ShaderBuilder>>) -> Result<ResourceId<Box<dyn Font>>, String> {
        let shader_builder_to_use = shader_builder.unwrap_or(
            bitmap_builder.default_shader_builder()
        );

        let mut hasher = DefaultHasher::new();
        shader_builder_to_use.hash().hash(&mut hasher);
        bitmap_builder.get_hash().hash(&mut hasher);
        let hash = hasher.finish();

        match self.fonts.get_by_builder_hash(&hash) {
            Some(existing) => return Ok(existing),
            None => (),
        }

        let shader_id = self.load_shader(shader_builder_to_use)?;
        let font = GlFont::new(bitmap_builder, shader_id, self)?;

        self.fonts.add(Box::new(font), hash)
    }

    fn get_font_by_id(&mut self, id: &ResourceId<Box<dyn Font>>) -> Option<&Box<dyn Font>> {
        self.fonts.get_asset_by_id(id)
    }

    fn load_shader(&mut self, shader_builder: Box<dyn ShaderBuilder>) -> Result<ResourceId<Box<dyn ShaderProgram>>, String> {
        let hash = shader_builder.hash()?;

        match self.shaders.get_by_builder_hash(&hash) {
            Some(existing) => return Ok(existing),
            None => (),
        }

        let shader = shader_builder.build()?;
        
        self.shaders.add(shader, hash)
    }

    fn get_shader_by_id(&mut self, id: &ResourceId<Box<dyn ShaderProgram>>) -> Option<&Box<dyn ShaderProgram>> {
        self.shaders.get_asset_by_id(id)
    }

    /// Create a new material. We do not check for existing materials with the same hash because each
    /// objet in the world will need a separate material.
    /// 
    /// If we want to have a builder hash in the feature, we probably want to add all parameters to it, which
    /// is only the shader_id at the time of writing.
    fn load_material(&mut self, shader_id: &ResourceId<Box<dyn ShaderProgram>>) -> Result<ResourceId<Material>, String> {
        let material = Material::new(shader_id.duplicate());
        self.materials.add(material, false)
    }

    fn get_material_by_id(&mut self, id: &ResourceId<Material>) -> Option<&mut Material> {
        self.materials.get_mut_asset_by_id(id)
    }

    fn add_material_texture(&mut self, material_id: &ResourceId<Material>, texture_id: &ResourceId<Box<dyn Texture>>) -> Result<(), String> {
        let textures_length: usize;
        let shader_id;
        match self.get_material_by_id(material_id) {
            Some(material) => {
                textures_length = material.number_of_textures();
                shader_id = material.shader_id.duplicate();
                material.push_texture_id(texture_id.duplicate());

            },
            None => return Err(format!("Material {} not found", material_id.id())),
        }
        
        match self.get_shader_by_id(&shader_id) {
            Some(shader) => {
                shader.set_uniform(
                    format!("texture{}", textures_length).as_str(), 
                    &UniformValue::from(textures_length as i32)
                );
            },
            None => return Err(format!("Shader {} from material {} was not found", shader_id.id(), material_id.id())),
        }

        Ok(())
    }

    fn activate_material(&mut self, material_id: &ResourceId<Material>) -> Result<(), String> {
        // apply shader
        match self.get_material_shader(material_id) {
            Some(material) => {
                material.apply();
            },
            None => return Err(format!("Shader for material {} not found", material_id.id())),
        }

        // activate textures
        let texture_ids;
        match self.get_material_by_id(material_id) {
            Some(material) => {
                texture_ids = material.texture_ids_copy();
            },
            None => return Err(format!("Material {} not found", material_id.id())),
        }

        for (unit, texture_id) in texture_ids.iter().enumerate() {
            match self.get_texture_by_id(texture_id) {
                Some(texture) => {
                    texture.activate(unit)
                },
                None => return Err(format!("Texture {} from material {} was not found", texture_id.id(), material_id.id())),
            }
        }
    
        Ok(())
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

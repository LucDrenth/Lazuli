use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use crate::{graphics::{texture::{Texture, TextureImage}, font::{Font, BitmapBuilder}, shader::{ShaderBuilder, ShaderProgram}, material::Material}, ResourceId};

use super::asset_collection::AssetCollection;

pub struct AssetManager {
    textures: AssetCollection<Texture, Option<String>>,
    fonts: AssetCollection<Font, u64>,
    shaders: AssetCollection<ShaderProgram, u64>,
    materials: AssetCollection<Material, u64>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: AssetCollection::new(),
            fonts: AssetCollection::new(),
            shaders: AssetCollection::new(),
            materials: AssetCollection::new(),
        }
    }

    pub fn load_texture(&mut self, path: impl Into<String>) -> Result<ResourceId<Texture>, String> {
        let path_string = path.into();

        let some_path = Some(path_string.clone());

        match self.textures.get_by_builder_hash(&some_path) {
            Some(existing) => {
                return Ok(existing)
            },
            None => (),
        }

        let texture = Texture::new();
        match texture.load_from_path(path_string) {
            Ok(_) => (),
            Err(err) => {
                return Err(err);
            },
        }

        self.textures.add(texture, some_path)
    }

    pub fn load_texture_from_image<T: Into<TextureImage>>(&mut self, img: T) -> Result<ResourceId<Texture>, String> {
        let texture = Texture::new();
        texture.load_from_image(img);

        self.textures.add(texture, None)
    }

    pub fn get_texture_by_id(&mut self, id: &ResourceId<Texture>) -> Option<&Texture> {
        self.textures.get_asset_by_id(id)
    }

    pub fn load_font(&mut self, bitmap_builder: impl BitmapBuilder, shader_builder: Option<ShaderBuilder>) -> Result<ResourceId<Font>, String> {
        let shader_builder_to_use: ShaderBuilder = match shader_builder {
            Some(builder) => builder,
            None => bitmap_builder.default_shader_builder(),
        };

        let mut hasher = DefaultHasher::new();
        shader_builder_to_use.hash().hash(&mut hasher);
        bitmap_builder.get_hash().hash(&mut hasher);
        let hash = hasher.finish();

        match self.fonts.get_by_builder_hash(&hash) {
            Some(existing) => return Ok(existing),
            None => (),
        }

        let shader_id = self.load_shader(shader_builder_to_use)?;
        let font = Font::new(bitmap_builder, shader_id, self)?;

        self.fonts.add(font, hash)
    }

    pub fn get_font_by_id(&mut self, id: &ResourceId<Font>) -> Option<&Font> {
        self.fonts.get_asset_by_id(id)
    }

    pub fn load_shader(&mut self, shader_builder: ShaderBuilder) -> Result<ResourceId<ShaderProgram>, String> {
        let hash = shader_builder.hash()?;

        match self.shaders.get_by_builder_hash(&hash) {
            Some(existing) => return Ok(existing),
            None => (),
        }

        let shader = shader_builder.build()?;
        
        self.shaders.add(shader, hash)
    }

    pub fn get_shader_by_id(&mut self, id: &ResourceId<ShaderProgram>) -> Option<&ShaderProgram> {
        self.shaders.get_asset_by_id(id)
    }

    pub fn load_material(&mut self, shader_id: &ResourceId<ShaderProgram>) -> Result<ResourceId<Material>, String> {
        // All parameters of this function must be put in this hash
        let mut hasher = DefaultHasher::new();
        shader_id.id().hash(&mut hasher);
        let hash = hasher.finish();

        match self.materials.get_by_builder_hash(&hash) {
            Some(existing) => return Ok(existing),
            None => (),
        }

        let material = Material::new(shader_id.duplicate());
        
        self.materials.add(material, hash)
    }

    pub fn get_material_by_id(&mut self, id: &ResourceId<Material>) -> Option<&mut Material> {
        self.materials.get_mut_asset_by_id(id)
    }

    pub fn add_material_texture(&mut self, material_id: &ResourceId<Material>, texture_id: &ResourceId<Texture>) {
        {
            let textures_length = self.get_material_by_id(material_id).unwrap().number_of_textures();
            let shader_id = self.get_material_by_id(material_id).unwrap().shader_id.duplicate();
            let shader = self.get_shader_by_id(&shader_id).unwrap();
            
            shader.set_uniform(
                format!("texture{}", textures_length).as_str(), 
                textures_length as i32
            );
        }

        self.get_material_by_id(material_id).unwrap().push_texture_id(texture_id.duplicate());
    }

    pub fn activate_material(&mut self, material_id: &ResourceId<Material>) {
        let texture_ids = self.get_material_by_id(material_id).unwrap().texture_ids_copy();

        for (index, texture_id) in texture_ids.iter().enumerate() {
            self.get_texture_by_id(texture_id).unwrap().activate(index);
        }
    }

    pub fn get_material_shader(&mut self, material_id: &ResourceId<Material>) -> Option<&ShaderProgram> {
        let shader_id;

        match self.get_material_by_id(material_id) {
            Some(material) => shader_id = material.shader_id.duplicate(),
            None => return None,
        }

        self.get_shader_by_id(&shader_id)
    }
}

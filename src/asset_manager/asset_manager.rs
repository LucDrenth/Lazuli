use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use crate::{graphics::{font::{BitmapBuilder, Font}, material::Material, shader::{ShaderBuilder, ShaderProgram}, texture::{GlTexture, Texture, TextureImage}}, ResourceId};

use super::asset_collection::AssetCollection;

pub trait AssetManagerTrait {
    fn load_texture(&mut self, path: impl Into<String>) -> Result<ResourceId<Box<dyn Texture>>, String>;
    fn load_texture_from_image(&mut self, texture_image: &dyn TextureImage) -> Result<ResourceId<Box<dyn Texture>>, String>;
    fn get_texture_by_id(&mut self, id: &ResourceId<Box<dyn Texture>>) -> Option<&Box<dyn Texture>>;
    fn load_font(&mut self, bitmap_builder: impl BitmapBuilder, shader_builder: Option<ShaderBuilder>) -> Result<ResourceId<Font>, String>;
    fn get_font_by_id(&mut self, id: &ResourceId<Font>) -> Option<&Font>;
    fn load_shader(&mut self, shader_builder: ShaderBuilder) -> Result<ResourceId<ShaderProgram>, String>;
    fn get_shader_by_id(&mut self, id: &ResourceId<ShaderProgram>) -> Option<&ShaderProgram>;
    fn load_material(&mut self, shader_id: &ResourceId<ShaderProgram>) -> Result<ResourceId<Material>, String>;
    fn get_material_by_id(&mut self, id: &ResourceId<Material>) -> Option<&mut Material>;
    fn add_material_texture(&mut self, material_id: &ResourceId<Material>, texture_id: &ResourceId<Box<dyn Texture>>);
    fn activate_material(&mut self, material_id: &ResourceId<Material>);
    fn get_material_shader(&mut self, material_id: &ResourceId<Material>) -> Option<&ShaderProgram>;
}

pub struct AssetManager {
    textures: AssetCollection<Box<dyn Texture>, Option<String>>,
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
}

impl AssetManagerTrait for AssetManager {
    fn load_texture(&mut self, path: impl Into<String>) -> Result<ResourceId<Box<dyn Texture>>, String> {
        let path_string = path.into();

        let some_path = Some(path_string.clone());

        match self.textures.get_by_builder_hash(&some_path) {
            Some(existing) => {
                return Ok(existing)
            },
            None => (),
        }

        match GlTexture::new_from_path(path_string) {
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

    fn load_font(&mut self, bitmap_builder: impl BitmapBuilder, shader_builder: Option<ShaderBuilder>) -> Result<ResourceId<Font>, String> {
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

    fn get_font_by_id(&mut self, id: &ResourceId<Font>) -> Option<&Font> {
        self.fonts.get_asset_by_id(id)
    }

    fn load_shader(&mut self, shader_builder: ShaderBuilder) -> Result<ResourceId<ShaderProgram>, String> {
        let hash = shader_builder.hash()?;

        match self.shaders.get_by_builder_hash(&hash) {
            Some(existing) => return Ok(existing),
            None => (),
        }

        let shader = shader_builder.build()?;
        
        self.shaders.add(shader, hash)
    }

    fn get_shader_by_id(&mut self, id: &ResourceId<ShaderProgram>) -> Option<&ShaderProgram> {
        self.shaders.get_asset_by_id(id)
    }

    /// Create a new material. We do not check for existing materials with the same hash because each
    /// objet in the world will need a separate material.
    /// 
    /// If we want to have a builder hash in the feature, we probably want to add all parameters to it, which
    /// is only the shader_id at the time of writing.
    fn load_material(&mut self, shader_id: &ResourceId<ShaderProgram>) -> Result<ResourceId<Material>, String> {
        let material = Material::new(shader_id.duplicate());
        self.materials.add(material, 0)
    }

    fn get_material_by_id(&mut self, id: &ResourceId<Material>) -> Option<&mut Material> {
        self.materials.get_mut_asset_by_id(id)
    }

    fn add_material_texture(&mut self, material_id: &ResourceId<Material>, texture_id: &ResourceId<Box<dyn Texture>>) {
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

    fn activate_material(&mut self, material_id: &ResourceId<Material>) {
        // apply shader
        self.get_material_shader(material_id).unwrap().apply();

        // activate textures
        let texture_ids = self.get_material_by_id(material_id).unwrap().texture_ids_copy();

        for (index, texture_id) in texture_ids.iter().enumerate() {
            self.get_texture_by_id(texture_id).unwrap().activate(index);
        }
    }

    fn get_material_shader(&mut self, material_id: &ResourceId<Material>) -> Option<&ShaderProgram> {
        let shader_id;

        match self.get_material_by_id(material_id) {
            Some(material) => shader_id = material.shader_id.duplicate(),
            None => return None,
        }

        self.get_shader_by_id(&shader_id)
    }
}

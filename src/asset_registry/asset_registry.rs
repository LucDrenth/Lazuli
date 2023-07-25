use std::{collections::{HashMap, hash_map::DefaultHasher}, path::Path, hash::{Hash, Hasher}};

use crate::graphics::{texture::{Texture, TextureImage}, font::{Font, BitmapBuilder}, shader::{ShaderBuilder, ShaderProgram}, material::Material};

use super::AssetId;

struct TextureEntry {
    path: Option<String>,
    texture: Texture,
}
struct FontEntry {
    hash: u64, // a hash of the font builder and the shader builder
    font: Font,
}
struct ShaderEntry {
    hash: u64,
    shader: ShaderProgram,
}
struct MaterialEntry {
    hash: u64,
    material: Material,
}

pub struct AssetRegistry {
    textures: HashMap<u32, TextureEntry>,
    current_texture_id: u32,

    fonts: HashMap<u32, FontEntry>,
    current_font_id: u32,

    shaders: HashMap<u32, ShaderEntry>,
    current_shader_id: u32,

    materials: HashMap<u32, MaterialEntry>,
    current_material_id: u32,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            current_texture_id: 0,
            fonts: HashMap::new(),
            current_font_id: 0,
            shaders: HashMap::new(),
            current_shader_id: 0,
            materials: HashMap::new(),
            current_material_id: 0,
        }
    }

    pub fn load_texture(&mut self, path: String) -> Result<AssetId<Texture>, String> {
        for (existing_texture_id, existing_texture_entry) in self.textures.iter() {
            match &existing_texture_entry.path {
                Some(existing_texture_entry_path) => {
                    if *existing_texture_entry_path == path {
                        return Ok(AssetId::new(*existing_texture_id));
                    }
                },
                None => continue,
            }
        }

        let texture = Texture::new();
        match texture.load_from_path(Path::new(&path)) {
            Ok(_) => (),
            Err(err) => {
                return Err(err);
            },
        }

        self.add_texture(texture, Some(path))
    }

    pub fn add_texture_from_image<T: Into<TextureImage>>(&mut self, img: T) -> Result<AssetId<Texture>, String> {
        let texture = Texture::new();
        texture.load_from_image(img);

        self.add_texture(texture, None)
    }

    fn add_texture(&mut self, texture: Texture, path: Option<String>) ->  Result<AssetId<Texture>, String> {
        self.current_texture_id += 1;

        match self.textures.entry(self.current_texture_id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(format!("Encountered duplicate id {} while adding texture to asset registry", self.current_texture_id));
            },
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(TextureEntry { path, texture });
            },
        }

        Ok(AssetId::new(self.current_texture_id))
    }

    pub fn get_texture_by_id(&self, id: &AssetId<Texture>) -> Option<&Texture> {
        match self.textures.get(id.id()) {
            Some(entry) => Some(&entry.texture),
            None => None,
        }
    }

    pub fn load_font(&mut self, bitmap_builder: impl BitmapBuilder, shader_builder: Option<ShaderBuilder>) -> Result<AssetId<Font>, String> {
        let shader_builder_to_use: ShaderBuilder = match shader_builder {
            Some(builder) => builder,
            None => bitmap_builder.default_shader_builder(),
        };

        let mut hasher = DefaultHasher::new();
        shader_builder_to_use.hash().hash(&mut hasher);
        bitmap_builder.get_hash().hash(&mut hasher);
        let hash = hasher.finish();

        for (existing_font_id, existing_font_entry) in self.fonts.iter() {
            if existing_font_entry.hash == hash {
                return Ok(AssetId::new(*existing_font_id))
            }
        }

        let shader_id = self.load_shader(shader_builder_to_use)?;

        let font = Font::new(bitmap_builder, shader_id, self)?;

        self.current_font_id += 1;

        match self.fonts.entry(self.current_font_id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(format!("Encountered duplicate id {} while adding font to asset registry", self.current_font_id));
            },
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(FontEntry { hash, font });
            },
        }

        Ok(AssetId::new(self.current_font_id))
    }

    pub fn get_font_by_id(&mut self, id: &AssetId<Font>) -> Option<&Font> {
        match self.fonts.get(id.id()) {
            Some(entry) => Some(&entry.font),
            None => None,
        }
    }

    pub fn load_shader(&mut self, shader_builder: ShaderBuilder) -> Result<AssetId<ShaderProgram>, String> {
        let hash = shader_builder.hash()?;

        for (existing_shader_id, existing_shader_entry) in self.shaders.iter() {
            if existing_shader_entry.hash == hash {
                return Ok(AssetId::new(*existing_shader_id));
            }
        }

        let shader = shader_builder.build()?;
        self.current_shader_id += 1;

        match self.shaders.entry(self.current_shader_id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(format!("Encountered duplicate id {} while adding shader to asset registry", self.current_shader_id));
            },
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(ShaderEntry { hash, shader });
            },
        }

        Ok(AssetId::new(self.current_shader_id))
    }

    pub fn get_shader_by_id(&self, id: &AssetId<ShaderProgram>) -> Option<&ShaderProgram> {
        match self.shaders.get(&id.id()) {
            Some(entry) => Some(&entry.shader),
            None => None,
        }
    }

    pub fn load_material(&mut self, shader_id: &AssetId<ShaderProgram>) -> Result<AssetId<Material>, String> {
        // All parameters of this function must be put in this hash
        let mut hasher = DefaultHasher::new();
        shader_id.id().hash(&mut hasher);
        let hash = hasher.finish();

        for (existing_material_id, existing_material_entry) in self.materials.iter() {
            if existing_material_entry.hash == hash {
                return Ok(AssetId::new(*existing_material_id));
            }
        }

        let material = Material::new(shader_id.duplicate());
        self.current_material_id += 1;

        match self.materials.entry(self.current_material_id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(format!("Encountered duplicate id {} while adding material to asset registry", self.current_material_id));
            },
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(MaterialEntry { hash, material });
            },
        }

        Ok(AssetId::new(self.current_material_id))
    }

    pub fn get_material_by_id(&mut self, id: &AssetId<Material>) -> Option<&mut Material> {
        match self.materials.get_mut(id.id()) {
            Some(entry) => {
                Some(&mut entry.material)
            },
            None => None,
        }
    }

    pub fn add_material_texture(&mut self, material_id: &AssetId<Material>, texture_id: &AssetId<Texture>) {
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

    pub fn activate_material(&mut self, material_id: &AssetId<Material>) {
        let texture_ids = self.get_material_by_id(material_id).unwrap().texture_ids_copy();

        for (index, texture_id) in texture_ids.iter().enumerate() {
            self.get_texture_by_id(texture_id).unwrap().activate(index);
        }
    }

    pub fn get_material_shader(&mut self, material_id: &AssetId<Material>) -> Option<&ShaderProgram> {
        let shader_id;

        match self.get_material_by_id(material_id) {
            Some(material) => shader_id = material.shader_id.duplicate(),
            None => return None,
        }

        self.get_shader_by_id(&shader_id)
    }
}

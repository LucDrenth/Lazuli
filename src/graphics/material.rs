use crate::asset_manager::{AssetManager, AssetId};

use super::{shader::ShaderProgram, texture::Texture};

pub struct Material {
    pub shader_id: AssetId<ShaderProgram>,
    texture_ids: Vec<AssetId<Texture>>,
}

impl Material {
    pub fn new(shader_id: AssetId<ShaderProgram>) -> Self {
        Self {
            shader_id,
            texture_ids: vec![],
        }
    }

    pub fn add_texture(&mut self, texture_id: AssetId<Texture>, asset_manager: &mut AssetManager) {
        asset_manager.get_shader_by_id(&self.shader_id).unwrap().set_uniform(
            format!("texture{}", self.texture_ids.len()).as_str(), 
            self.texture_ids.len() as i32
        );

        self.texture_ids.push(texture_id);
    }

    pub fn push_texture_id(&mut self, texture_id: AssetId<Texture>) {
        self.texture_ids.push(texture_id);
    }

    pub fn activate(&self, asset_manager: &mut AssetManager) {
        asset_manager.get_shader_by_id(&self.shader_id).unwrap().apply();
        
        for (index, texture_id) in self.texture_ids.iter().enumerate() {
            asset_manager.get_texture_by_id(texture_id).unwrap().activate(index);
        }
    }

    pub fn shader<'a>(&'a self, asset_manager: &'a mut AssetManager) -> Option<&ShaderProgram> {
        asset_manager.get_shader_by_id(&self.shader_id)
    }

    pub fn texture_ids_copy(&self) -> Vec<AssetId<Texture>> {
        let mut texture_ids_clone: Vec<AssetId<Texture>> = Vec::with_capacity(self.texture_ids.len());

        for texture_id in self.texture_ids.iter() {
            texture_ids_clone.push(texture_id.duplicate())
        }

        texture_ids_clone
    }

    pub fn number_of_textures(&self) -> usize {
        self.texture_ids.len()
    }
}

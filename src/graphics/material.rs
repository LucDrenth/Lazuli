use crate::asset_registry::AssetRegistry;

use super::shader::ShaderProgram;

pub struct Material {
    pub shader_id: u32,
    texture_ids: Vec<u32>,
}

impl Material {
    pub fn new(shader_id: u32) -> Self {
        Self {
            shader_id,
            texture_ids: vec![],
        }
    }

    pub fn add_texture(&mut self, texture_id: u32, asset_registry: &mut AssetRegistry) {
        asset_registry.get_shader_by_id(self.shader_id).unwrap().set_uniform(
            format!("texture{}", self.texture_ids.len()).as_str(), 
            self.texture_ids.len() as i32
        );

        self.texture_ids.push(texture_id);
    }

    pub fn push_texture_id(&mut self, texture_id: u32) {
        self.texture_ids.push(texture_id);
    }

    pub fn activate(&self, asset_registry: &mut AssetRegistry) {
        asset_registry.get_shader_by_id(self.shader_id).unwrap().apply();
        
        for (index, texture_id) in self.texture_ids.iter().enumerate() {
            asset_registry.get_texture_by_id(*texture_id).unwrap().activate(index);
        }
    }

    pub fn shader<'a>(&'a self, asset_registry: &'a mut AssetRegistry) -> Option<&ShaderProgram> {
        asset_registry.get_shader_by_id(self.shader_id)
    }

    pub fn texture_ids_copy(&self) -> Vec<u32> {
        self.texture_ids.clone()
    }

    pub fn number_of_textures(&self) -> usize {
        self.texture_ids.len()
    }
}

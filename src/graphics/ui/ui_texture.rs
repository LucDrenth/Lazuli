use crate::{graphics::{texture::Texture, material::Material}, ResourceId, asset_manager::AssetManager};

pub enum UiTexture {
    Id(ResourceId<Texture>),
    Path(String),
}

impl UiTexture {
    pub fn upload(&self, material_id: &ResourceId<Material>, asset_manager: &mut AssetManager) -> Result<ResourceId<Texture>, String> {
        match self {
            UiTexture::Id(texture_id) => {
                asset_manager.add_material_texture(&material_id, &texture_id);
                Ok(texture_id.duplicate())
            },
            UiTexture::Path(texture_path) => {
                match asset_manager.load_texture(texture_path) {
                    Ok(texture_id) => {
                        asset_manager.add_material_texture(&material_id, &texture_id);
                        Ok(texture_id)
                    },
                    Err(err) => {
                        Err(err)
                    },
                }
            },
        }
    }
}

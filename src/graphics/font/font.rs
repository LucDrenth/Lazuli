use std::{fs::File, io::Read};

use crate::{asset_manager::AssetManager, graphics::{material::Material, shader::ShaderProgram, texture::GlTextureImage}, log, ResourceId};

use super::{Bitmap, bitmap::BitmapBuilder, bitmap_cache};

pub trait Font {
    fn atlas(&self) -> &Box<dyn Bitmap>;
    fn line_height(&self) -> f32;
    fn space_size(&self) -> f32;
    fn bitmap_spread(&self) -> u8;
    fn get_material_id(&self) -> &ResourceId<Material>;
}

pub struct GlFont {
    atlas: Box<dyn Bitmap>,

    /// The width of the space (' ') character. The space is relative to the line height. So 0.5 is halve the line height. 
    pub space_size: f32,
    pub material_id: ResourceId<Material>,
}

impl Font for GlFont {
    fn atlas(&self) -> &Box<dyn Bitmap> {
        &self.atlas
    }

    fn line_height(&self) -> f32 {
        self.atlas.line_height()
    }

    fn space_size(&self) -> f32 {
        self.space_size
    }

    fn bitmap_spread(&self) -> u8 {
        self.atlas.spread()
    }

    fn get_material_id(&self) -> &ResourceId<Material> {
        &self.material_id
    }
}

impl GlFont {
    pub fn new(bitmap_builder: &dyn BitmapBuilder, shader_id: ResourceId<Box<dyn ShaderProgram>>, asset_manager: &mut dyn AssetManager) -> Result<Self, String> {
        match load_font(bitmap_builder.font_file_path()) {
            Ok(font) => {
                let atlas = Self::get_bitmap(font, bitmap_builder.font_file_path(), bitmap_builder)?;

                let texture_image: GlTextureImage = atlas.image().into();
                let texture_id = asset_manager.load_texture_from_image(&texture_image)?;
                let material_id = asset_manager.load_material(&shader_id)?;
                asset_manager.add_material_texture(&material_id, &texture_id);

                Ok(Self { 
                    atlas,
                    space_size: 0.215,
                    material_id,
                })
            },
            Err(err) => {
                log::engine_err(format!("Failed to create font from path {}: {}", bitmap_builder.font_file_path(), err));
                Err(err)
            },
        }
    }

    fn get_bitmap(font: rusttype::Font<'static>, path: &String, bitmap_builder: &dyn BitmapBuilder) -> Result<Box<dyn Bitmap>, String> {
        if let Some(existing_bitmap) = bitmap_cache::load(&path, bitmap_builder) {
            return Ok(existing_bitmap)
        } else {
            let new_bitmap = bitmap_builder.build(&font)?;

            if bitmap_builder.do_cache() {
                match bitmap_cache::save(path, bitmap_builder, &new_bitmap) {
                    Ok(_) => (),
                    Err(err) => {
                        log::engine_warn(format!("Failed to save sdf font bitmap cache: {}", err));
                    },
                }
            }

            return Ok(new_bitmap);
        }
    }
}

pub fn load_font(path: &String) -> Result<rusttype::Font<'static>, String> {
    let mut file = File::open(path).map_err(|err| {
        format!("Failed to open file: {}", err.to_string())
    })?;

    let mut font_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut font_data).map_err(|err| {
        format!("Failed to read file: {}", err.to_string())
    })?;

    match rusttype::Font::try_from_vec(font_data) {
        Some(font) => Ok(font),
        None => Err(format!("Failed to create Font")),
    }
}

use std::{fs::File, io::Read, collections::HashMap};

use crate::{graphics::{texture::ImageType, shader::ShaderProgram, material::Material}, asset_manager::AssetManager, log, ResourceId};

use super::{BitmapCharacter, Bitmap, bitmap::BitmapBuilder, bitmap_cache};

pub struct Font {
    bitmap: Box<dyn Bitmap>,

    /// The width of the space (' ') character. the space is relative to the line height. So 0.5 is halve the line height. 
    pub space_size: f32,
    pub material_id: ResourceId<Material>,
}

/// # Arguments
/// 
/// * `path` - Path to a .ttf file
/// * `bitmap_builder` -
/// * `shader` - None to use the default text shader
impl Font {
    pub fn new(bitmap_builder: impl BitmapBuilder, shader_id: ResourceId<ShaderProgram>, asset_manager: &mut AssetManager) -> Result<Self, String> {
        match load_font(bitmap_builder.font_file_path()) {
            Ok(font) => {
                let bitmap = Self::get_bitmap(font, bitmap_builder.font_file_path(), &bitmap_builder)?;

                let texture_id = asset_manager.load_texture_from_image(bitmap.image())?.duplicate();
                let material_id = asset_manager.load_material(&shader_id)?.duplicate();
                asset_manager.add_material_texture(&material_id, &texture_id);

                Ok(Self { 
                    bitmap,
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

    pub fn save_bitmap(&self, path: String) -> Result<(), String> {
        self.bitmap.save(&path)
    }

    pub fn image(&self) -> &ImageType {
        &self.bitmap.image()
    }

    pub fn get_bitmap_character(&self, character: char) -> Option<&BitmapCharacter> {
        self.bitmap.characters().get(&character)
    }

    pub fn bitmap_characters_copy(&self) -> HashMap<char, BitmapCharacter> {
        self.bitmap.characters().clone()
    }

    pub fn line_height(&self) -> f32 {
        self.bitmap.line_height()
    }

    pub fn bitmap_spread(&self) -> u8 {
        self.bitmap.spread()
    }

    // pub fn material<'a>(&'a self, asset_manager: &'a AssetManager) -> Option<&mut Material> {
    //     asset_manager.get_material_by_id(self.material_id)
    // }

    fn get_bitmap(font: rusttype::Font<'static>, path: &String, bitmap_builder: &impl BitmapBuilder) -> Result<Box<dyn Bitmap>, String> {
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

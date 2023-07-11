use std::{fs::File, io::Read};

use crate::{lz_core_err, lz_core_warn, graphics::texture::ImageType};

use super::{SdfBitmapBuilder, sdf_bitmap_cache, BitmapCharacter, sdf_bitmap::SdfBitmap, Bitmap};

pub struct Font {
    bitmap: Box<dyn Bitmap>,

    /// The width of the space (' ') character. the space is relative to the line height. So 0.5 is halve the line height. 
    pub space_size: f32,
}

impl Font {
    pub fn new(path: String, bitmap_builder: SdfBitmapBuilder) -> Result<Self, String> {
        match load_font(&path) {
            Ok(font) => {
                Ok(Self { 
                    bitmap: Self::get_bitmap(font, &path, &bitmap_builder)?,
                    space_size: 0.215,
                })
            },
            Err(err) => {
                lz_core_err!("Failed to create font from path {}: {}", path, err);
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

    pub fn line_height(&self) -> f32 {
        self.bitmap.line_height()
    }

    pub fn bitmap_spread(&self) -> u8 {
        self.bitmap.spread()
    }

    fn get_bitmap(font: rusttype::Font<'static>, path: &String, bitmap_builder: &SdfBitmapBuilder) -> Result<Box<dyn Bitmap>, String> {
        if let Some(existing_bitmap) = sdf_bitmap_cache::load(&path, &bitmap_builder) {
            return Ok(Box::new(existing_bitmap));
        } else {
            let bitmap = SdfBitmap::new(&font, &bitmap_builder)?;

            if bitmap_builder.cache {
                match sdf_bitmap_cache::save(&path, &bitmap_builder, &bitmap) {
                    Ok(_) => (),
                    Err(err) => {
                        lz_core_warn!("Failed to save sdf font bitmap cache: {}", err); 
                    }
                }
            }

            return Ok(Box::new(bitmap));
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

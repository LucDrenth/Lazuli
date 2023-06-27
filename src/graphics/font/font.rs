use std::{fs::File, io::Read};

use image::RgbaImage;

use crate::lz_core_err;

use super::{bitmap::{Bitmap, BitmapBuilder}, BitmapCharacter};

pub struct Font {
    bitmap: Bitmap,
}

impl Font {
    pub fn new(path: String, bitmap_builder: BitmapBuilder) -> Result<Self, String> {
        match load_font(&path) {
            Ok(font) => {
                let bitmap = Bitmap::new(&font, bitmap_builder)?;
                bitmap.save(&format!("{}.bitmap.png", path))?;

                Ok(Self { 
                    bitmap
                })
            },
            Err(err) => {
                lz_core_err!("Failed to create font from path {}: {}", path, err);
                Err(err)
            },
        }
    }

    pub fn image(&self) -> &RgbaImage {
        &self.bitmap.image
    }

    pub fn get_bitmap_character(&self, character: char) -> Option<&BitmapCharacter> {
        self.bitmap.characters.get(&character)
    }
}

fn load_font(path: &String) -> Result<rusttype::Font<'static>, String> {
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

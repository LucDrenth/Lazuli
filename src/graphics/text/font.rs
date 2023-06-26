use std::{fs::File, io::Read};

use crate::lz_core_err;

use super::bitmap::{Bitmap, BitmapBuilder};

pub struct Font {
    pub font_size: f32,
}

impl Font {
    pub fn new(path: String, font_size: f32) -> Result<Self, String> {
        match load_font(&path) {
            Ok(font) => {
                let bitmap = Bitmap::new(&font, BitmapBuilder::new().with_font_size(60.0))?;
                bitmap.save(&format!("{}.bitmap.png", path))?;
            },
            Err(err) => {
                lz_core_err!("Failed to create font from path {}: {}", path, err);
            },
        }

        Ok(Self { 
            font_size
        })
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

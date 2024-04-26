use std::collections::HashMap;

use crate::graphics::texture::ImageType;

use super::{Bitmap, BitmapCharacter};

pub struct MockBitmap {
    pub characters: HashMap<char, BitmapCharacter>,
    pub line_height: f32,
    pub spread: u8,
    pub json_cache: String,
}

impl Bitmap for MockBitmap {
    fn image(&self) -> &ImageType {
        &ImageType::Mock()
    }

    fn save(&self, _path: &String) -> Result<(), String> {
        Ok(())
    }

    fn characters(&self) -> &HashMap<char, BitmapCharacter> {
        &self.characters
    }

    fn line_height(&self) -> f32 {
        self.line_height
    }

    fn spread(&self) -> u8 {
        self.spread
    }

    fn to_json_cache(&self) -> Result<String, String> {
        Ok(self.json_cache.clone())
    }
}

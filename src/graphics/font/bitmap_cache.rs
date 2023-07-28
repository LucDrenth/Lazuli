use std::{fs, collections::{hash_map::DefaultHasher, HashMap}, hash::{Hash, Hasher}, path::Path};

use image::GrayImage;
use serde::{Serialize, Deserialize};

use crate::{graphics::texture::ImageType, log};

use super::{SdfBitmapBuilder, BitmapCharacter, sdf_bitmap::SdfBitmap, Bitmap, plain_bitmap::PlainBitmap, bitmap::BitmapBuilder};

pub trait BitmapCache {
    fn build(&self, texture: GrayImage) -> Box<dyn Bitmap>;
}

#[derive(Serialize, Deserialize)]
pub struct SdfBitmapCache {
    pub characters: HashMap<char, BitmapCharacter>,
    pub line_height: f32,
    pub spread: u8,
}

impl BitmapCache for SdfBitmapCache {
    fn build(&self, texture: GrayImage) -> Box<dyn Bitmap> {
        let mut characters: HashMap<char, BitmapCharacter> = HashMap::new();
        characters.clone_from(&self.characters);

        return Box::new(SdfBitmap {
            image: ImageType::GrayImage(texture),
            characters: characters,
            line_height: self.line_height,
            spread: self.spread,
        });
    }
}

impl SdfBitmapCache {
    pub fn from(bitmap: &SdfBitmap) -> Self {
        let mut bitmap_cache_characters: HashMap<char, BitmapCharacter> = HashMap::new();
        bitmap_cache_characters.clone_from(&bitmap.characters());
    
        Self {
            characters: bitmap_cache_characters,
            line_height: bitmap.line_height,
            spread: bitmap.spread,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct PlainBitmapCache {
    pub characters: HashMap<char, BitmapCharacter>,
    pub line_height: f32,
}

impl BitmapCache for PlainBitmapCache {
    fn build(&self, texture: GrayImage) -> Box<dyn Bitmap> {
        let mut characters: HashMap<char, BitmapCharacter> = HashMap::new();
        characters.clone_from(&self.characters);

        return Box::new(PlainBitmap {
            image: ImageType::GrayImage(texture),
            characters: characters,
            line_height: self.line_height,
        });
    }
}

impl PlainBitmapCache {
    pub fn from(bitmap: &PlainBitmap) -> Self {
        let mut bitmap_cache_characters: HashMap<char, BitmapCharacter> = HashMap::new();
        bitmap_cache_characters.clone_from(&bitmap.characters());
    
        Self {
            characters: bitmap_cache_characters,
            line_height: bitmap.line_height,
        }
    }
}


pub fn load(font_path: &String, bitmap_builder: &impl BitmapBuilder) -> Option<Box<dyn Bitmap>> {
    let bitmap_builder_hash: String;
    match bitmap_builder.get_hash() {
        Ok(hash) => bitmap_builder_hash = hash,
        Err(err) => {
            log::engine_err(format!("failed to check sdf bitmap cache because bitmap_builder hash could not be created: {}", err));
            return None;
        },
    }

    let bitmap_cache = read_character_data(bitmap_builder, &font_path, &bitmap_builder_hash)?;
    let texture: GrayImage = read_texture(&font_path, &bitmap_builder_hash)?;

    return Some(bitmap_cache.build(texture));
}

pub fn save(font_path: &String, bitmap_builder: &impl BitmapBuilder, bitmap: &Box<dyn Bitmap>) -> Result<(), String> {
    let bitmap_builder_hash = bitmap_builder.get_hash()?;

    bitmap.save(&format!("{}.{}-texture.png", font_path, bitmap_builder_hash))?;

    let json: String = bitmap.to_json_cache()?;
    let path = format!("{}.{}-data.json", font_path, bitmap_builder_hash);
    fs::write(Path::new(&path), json).map_err(|err| {
        format!("failed to save bitmap cache to file: {}", err)
    })?;

    Ok(())
}

fn get_bitmap_builder_hash(bitmap_builder: &SdfBitmapBuilder) -> Result<String, String> {
    match serde_json::to_string(bitmap_builder) {
        Ok(bitmap_builder_string) => {
            let mut hasher = DefaultHasher::new();
            bitmap_builder_string.hash(&mut hasher);
            Ok(hasher.finish().to_string())
        },
        Err(err) => Err(err.to_string()),
    }
}

fn read_character_data(bitmap_builder: &impl BitmapBuilder, font_path: &String, bitmap_builder_hash: &String) -> Option<Box<dyn BitmapCache>> {
    match fs::read_to_string(format!("{}.{}-data.json", font_path, bitmap_builder_hash)) {
        Ok(data) => {
            return bitmap_builder.cache_from_json(data);
        },
        Err(_) => {
            // cache does not exist
            return None;
        },
    }
}

fn read_texture(font_path: &String, bitmap_builder_hash: &String) -> Option<GrayImage> {
    match image::open(format!("{}.{}-texture.png", font_path, bitmap_builder_hash)) {
        Ok(image) => {
            Some(image.into_luma8())
        },
        Err(_) => {
            // cache does not exist
            return None;
        },
    }
}

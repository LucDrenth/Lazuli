use std::{fs, collections::{hash_map::DefaultHasher, HashMap}, hash::{Hash, Hasher}, path::Path};

use image::GrayImage;
use serde::{Serialize, Deserialize};

use crate::{lz_core_err, graphics::texture::ImageType};

use super::{SdfBitmapBuilder, BitmapCharacter, sdf_bitmap::SdfBitmap, Bitmap};

#[derive(Serialize, Deserialize)]
pub struct SdfBitmapCache {
    pub characters: HashMap<char, BitmapCharacter>,
    pub line_height: f32,
    pub spread: u8,
}

impl SdfBitmapCache {
    pub fn from(bitmap: &SdfBitmap) -> Self {
        let mut bitmap_cache_characters: HashMap<char, BitmapCharacter> = HashMap::new();
        bitmap_cache_characters.clone_from(&bitmap.characters());
    
        SdfBitmapCache {
            characters: bitmap_cache_characters,
            line_height: bitmap.line_height,
            spread: bitmap.spread,
        }
    }
}

pub fn load(font_path: &String, bitmap_builder: &SdfBitmapBuilder) -> Option<SdfBitmap> {
    let bitmap_builder_hash: String;
    match get_bitmap_builder_hash(bitmap_builder) {
        Ok(hash) => bitmap_builder_hash = hash,
        Err(err) => {
            lz_core_err!("failed to check sdf bitmap cache because bitmap_builder hash could not be created: {}", err);
            return None;
        },
    }

    let bitmap_data = read_character_data(&font_path, &bitmap_builder_hash)?;
    let texture: GrayImage = read_texture(&font_path, &bitmap_builder_hash)?;

    return Some(SdfBitmap {
        image: ImageType::GrayImage(texture),
        characters: bitmap_data.characters,
        line_height: bitmap_data.line_height,
        spread: bitmap_data.spread,
    });
}

pub fn save(font_path: &String, bitmap_builder: &SdfBitmapBuilder, bitmap: &SdfBitmap) -> Result<(), String> {
    let bitmap_builder_hash = get_bitmap_builder_hash(bitmap_builder)?;

    bitmap.save(&format!("{}.{}-texture.png", font_path, bitmap_builder_hash))?;

    let bitmap_cache = SdfBitmapCache::from(bitmap);
    let json = serde_json::to_string(&bitmap_cache).map_err(|err| {
        format!("failed to serialize bitmap cache: {}", err)
    })?;
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

fn read_character_data(font_path: &String, bitmap_builder_hash: &String) -> Option<SdfBitmapCache> {
    match fs::read_to_string(format!("{}.{}-data.json", font_path, bitmap_builder_hash)) {
        Ok(data) => {
            match serde_json::from_str(&data) {
                Ok(cache) => return Some(cache),
                Err(err) => {
                    // cache exists but is not valid
                    lz_core_err!("Failed to read data from sdf bitmap cache: {}", err.to_string());
                    return None;
                },
            }
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

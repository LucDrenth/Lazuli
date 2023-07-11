use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use image::{DynamicImage, GrayImage, Luma};
use rusttype::PositionedGlyph;
use serde::Serialize;

use crate::{lz_core_warn, lz_core_err, graphics::texture::ImageType};

use super::{BitmapCharacter, Bitmap, bitmap::BitmapBuilder, bitmap_cache::PlainBitmapCache};

pub struct PlainBitmap {
    pub image: ImageType,
    pub characters: HashMap<char, BitmapCharacter>,
    pub line_height: f32,
}

impl Bitmap for PlainBitmap {
    fn image(&self) -> &crate::graphics::texture::ImageType {
        &self.image
    }

    fn save(&self, path: &String) -> Result<(), String> {
        self.image.save(path).map_err(|err| {
            format!("Failed to save bitmap image: {}", err)
        })
    }

    fn characters(&self) -> &HashMap<char, BitmapCharacter> {
        &self.characters
    }

    fn line_height(&self) -> f32 {
        self.line_height
    }

    fn spread(&self) -> u8 {
        0
    }

    fn to_json_cache(&self) -> Result<String, String> {
        let bitmap_cache = PlainBitmapCache::from(&self);

        serde_json::to_string(&bitmap_cache).map_err(|err| {
            format!("failed to serialize bitmap cache: {}", err)
        })
    }
}

impl PlainBitmap {
    pub fn new(font: &rusttype::Font<'static>, bitmap_builder: &PlainBitmapBuilder) -> Result<PlainBitmap, String> {
        if bitmap_builder.characters.len() == 0 {
            return Err("Failed to create bitmap: character set may not be empty".to_string());
        }

        let bitmap = Self::create(font, bitmap_builder).map_err(|err| {
            format!("Failed to create bitmap: {}", err)
        })?;
        Ok(bitmap)
    }

    fn create(font: &rusttype::Font<'static>, bitmap_builder: &PlainBitmapBuilder) -> Result<Self, String> {
        let scale = rusttype::Scale::uniform(bitmap_builder.font_size);
        let v_metrics = font.v_metrics(scale);
        let start_point = rusttype::point(bitmap_builder.padding_x as f32, bitmap_builder.padding_y as f32 + v_metrics.ascent);
        let glyphs: Vec<_> = font.layout(&bitmap_builder.characters, scale, start_point).collect();

        // TODO - order characters by glyphs height
        // TODO - use highest character of the row for that whole row instead of the line height
        // TODO - remove duplicates from bitmap_builder.characters (and add a warning if there is any duplicates)

        let line_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let (bitmap_width, bitmap_height) = calculate_image_size(&glyphs, line_height, bitmap_builder.padding_x, bitmap_builder.padding_y);

        let mut image_buffer: GrayImage = DynamicImage::new_luma8(bitmap_width, bitmap_height).to_luma8();
        let mut bitmap_characters: HashMap<char, BitmapCharacter> = HashMap::new();
        write_glyphs(
            glyphs, 
            &bitmap_builder.characters, 
            &mut image_buffer, 
            &mut bitmap_characters, 
            bitmap_builder.padding_x, 
            bitmap_builder.padding_y, 
            line_height as f32
        );

        Ok(Self{ 
            image: ImageType::GrayImage(image_buffer), 
            characters: bitmap_characters, 
            line_height: line_height as f32,
        })
    }

}

#[derive(Serialize)]
pub struct PlainBitmapBuilder {
    padding_x: u32,
    padding_y: u32,
    font_size: f32,
    characters: String,
    cache: bool,
}

impl BitmapBuilder for PlainBitmapBuilder {
    fn do_cache(&self) -> bool {
        self.cache
    }

    fn build(&self, font: &rusttype::Font<'static>) -> Result<Box<dyn Bitmap>, String> {
        match PlainBitmap::new(font, &self) {
            Ok(bitmap) => return Ok(Box::new(bitmap)),
            Err(err) => Err(err),
        }
    }

    fn get_hash(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(bitmap_builder_string) => {
                let mut hasher = DefaultHasher::new();
                bitmap_builder_string.hash(&mut hasher);
                Ok(hasher.finish().to_string())
            },
            Err(err) => Err(err.to_string()),
        }
    }

    fn cache_from_json(&self, data: String) -> Option<Box<dyn super::bitmap_cache::BitmapCache>> {
        let bitmap_cache: Result<PlainBitmapCache, serde_json::error::Error> = serde_json::from_str(&data);

        match bitmap_cache {
            Ok(cache) => return Some(Box::new(cache)),
            Err(err) => {
                // cache exists but is not valid
                lz_core_err!("Failed to read data from plain bitmap cache: {}", err.to_string());
                return None;
            },
        }
    }
}

impl PlainBitmapBuilder {
    pub fn new() -> Self {
        Self {
            padding_x: 0,
            padding_y: 0,
            font_size: 25.0,
            characters: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!;%:?*()_+-=.,/|\\\"'@#$€^&{}[]".to_string(),
            cache: true,
        }
    }

    pub fn with_padding_x(mut self, padding_x: u32) -> Self {
        self.padding_x = padding_x;
        self
    }

    pub fn with_padding_y(mut self, padding_y: u32) -> Self {
        self.padding_y = padding_y;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn with_characters(mut self, characters: String) -> Self {
        self.characters = characters;
        self
    }

    pub fn with_cache(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }
}

/// Calculate the size that the bitmap needs to be, as a powers of 2. such as 256x256, 512x512 etc.
fn calculate_image_size(glyphs: &Vec<PositionedGlyph<'_>>, line_height: u32, padding_x: u32, padding_y: u32,) -> (u32, u32) {
    // start at 128x128
    let mut current_width: u32 = 2_u32.pow(7);
    let mut current_height: u32 = 2_u32.pow(7); 

    let mut update_width = true; // if true, increase current_width. If false, increase current_height

    loop {
        if padding_x * 2 >= current_width || padding_y * 2 >= current_height {
            if update_width {
                current_width *= 2; 
                update_width = false;
            } else { 
                current_height *= 2; 
                update_width = true;
            }

            continue;
        }
        
        let width_to_fit = current_width - padding_x * 2;
        let height_to_fit = current_height - padding_y * 2;
        
        if glyphs_fit_in(width_to_fit, height_to_fit, &glyphs, line_height) {
            return (current_width, current_height)
        }

        if update_width {
            current_width *= 2; 
            update_width = false;
        } else { 
           current_height *= 2; 
           update_width = true;
        }
    }
}

/// Check if the glyphs fit within a certain image size
fn glyphs_fit_in(width: u32, height: u32, glyphs: &Vec<PositionedGlyph<'_>>, line_height: u32) -> bool {
    let mut current_x: u32 = 0;
    let mut current_y: u32 = 0;

    for glyph in glyphs.iter() {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            let character_width = (bounding_box.max.x - bounding_box.min.x) as u32;

            if current_x + character_width >= width {
                // go to next line
                current_x = character_width;
                current_y += line_height;

                if current_y + line_height >= height {
                    return false;
                }
            } else {
                current_x += character_width;
            }
        }
    }

    return true;
}

/// # Arguments
/// 
/// * `glyphs` - There exists a glyph for every character of the characters param, and no more
/// * `characters` - 
/// * `image_buffer` - 
/// * `bitmap_characters` - 
/// * `padding_x` - 
/// * `padding_y` - 
/// * `line_height` - 
fn write_glyphs(
    glyphs: Vec<PositionedGlyph<'_>>, 
    characters: &str,
    image_buffer: &mut GrayImage, 
    bitmap_characters: &mut HashMap<char, BitmapCharacter>,
    padding_x: u32,
    padding_y: u32,
    line_height: f32,
) {
    let mut current_x: u32 = padding_x;
    let mut current_y: u32 = padding_y;

    for (i, glyph) in glyphs.iter().enumerate() {
        let bitmap_width = image_buffer.width();
        let bitmap_height = image_buffer.height();

        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            let character: char = characters.chars().nth(i).take().unwrap();
            let character_width = (bounding_box.max.x - bounding_box.min.x) as u32;

            if current_x + character_width >= bitmap_width - padding_x {
                // go to next line
                current_x = padding_x;
                current_y += line_height as u32;

                if current_y >= bitmap_height - padding_y {
                    lz_core_err!("Failed to write glyphs to bitmap because it does not fit within the image");
                    return;
                }
            }

            match bitmap_characters.entry(character) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    lz_core_warn!("Encountered duplicate character [{}] while writing glyphs for characters [{}] to bitmap", character, characters);
                    continue;
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(BitmapCharacter {
                        texture_start_x: (current_x as i32) as f32 / bitmap_width as f32,
                        texture_end_x: (current_x + character_width) as f32 / bitmap_width as f32,
                        texture_start_y: current_y as f32 / bitmap_height as f32,
                        texture_end_y: (current_y as f32 + line_height) / bitmap_height as f32,
                        width: character_width as f32 / line_height,
                    });
                },
            }

            glyph.draw(|x, y, v| {
                image_buffer.put_pixel(
                    x + current_x,
                    y + current_y + bounding_box.min.y as u32 - padding_y,
                    Luma([(v * 255.0) as u8]),
                )
            });

            current_x += character_width;
        }
    }
}

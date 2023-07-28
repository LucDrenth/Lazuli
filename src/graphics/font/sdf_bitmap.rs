use std::collections::HashMap;

use image::{DynamicImage, Luma, GrayImage};
use rusttype::PositionedGlyph;

use crate::{math, graphics::texture::{downsample_gray_image, ImageType}, log};

use super::{BitmapCharacter, Bitmap, bitmap_cache::SdfBitmapCache, SdfBitmapBuilder};

/// Signed distance field font bitmap
pub struct SdfBitmap {
    pub image: ImageType,
    pub characters: HashMap<char, BitmapCharacter>,
    pub line_height: f32,
    pub spread: u8,
}

impl Bitmap for SdfBitmap {
    fn image(&self) -> &ImageType {
        &self.image
    }

    fn save(&self, path: &String) -> Result<(), String> {
        self.image.save(path).map_err(|err| {
            format!("Failed to save sdf bitmap image: {}", err)
        })
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
        let bitmap_cache = SdfBitmapCache::from(&self);

        serde_json::to_string(&bitmap_cache).map_err(|err| {
            format!("failed to serialize bitmap cache: {}", err)
        })
    }
}

impl SdfBitmap {
    pub fn new(font: &rusttype::Font<'static>, bitmap_builder: &SdfBitmapBuilder) -> Result<SdfBitmap, String> {
        if bitmap_builder.characters.len() == 0 {
            return Err("Failed to create bitmap: character set may not be empty".to_string());
        }

        let bitmap = Self::create(font, bitmap_builder).map_err(|err| {
            format!("Failed to create bitmap: {}", err)
        })?;
        Ok(bitmap)
    }

    fn create(font: &rusttype::Font<'static>, bitmap_builder: &SdfBitmapBuilder) -> Result<Self, String> {
        let scale = rusttype::Scale::uniform(bitmap_builder.font_size * bitmap_builder.super_sampling_factor as f32);
        let v_metrics = font.v_metrics(scale);
        let start_point = rusttype::point(bitmap_builder.padding_x as f32, bitmap_builder.padding_y as f32 + v_metrics.ascent);
        let glyphs: Vec<_> = font.layout(&bitmap_builder.characters, scale, start_point).collect();

        // TODO - order characters by glyphs height
        // TODO - use highest character of the row for that whole row instead of the line height
        // TODO - remove duplicates from bitmap_builder.characters (and add a warning if there is any duplicates)

        let line_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let (bitmap_width, bitmap_height) = calculate_image_size(&glyphs, line_height, &bitmap_builder);

        let mut image_buffer: GrayImage = DynamicImage::new_luma8(bitmap_width, bitmap_height).to_luma8();
        let mut bitmap_characters: HashMap<char, BitmapCharacter> = HashMap::new();
        write_glyphs(glyphs, &bitmap_builder, &mut image_buffer, &mut bitmap_characters, line_height as f32);

        let gray_image = downsample_gray_image(&image_buffer, bitmap_builder.super_sampling_factor as u32);

        Ok(Self{ 
            image: ImageType::GrayImage(gray_image), 
            characters: bitmap_characters, 
            line_height: line_height as f32,
            spread: bitmap_builder.spread,
        })
    }
}

/// Calculate the size that the bitmap needs to be, as a powers of 2. such as 256x256, 512x512 etc.
fn calculate_image_size(glyphs: &Vec<PositionedGlyph<'_>>, line_height: u32, bitmap_builder: &SdfBitmapBuilder) -> (u32, u32) {
    // start at 128x128
    let mut current_width: u32 = 2_u32.pow(7);
    let mut current_height: u32 = 2_u32.pow(7); 

    let mut update_width = true; // if true, increase current_width. If false, increase current_height

    loop {
        if bitmap_builder.padding_x * 2 >= current_width || bitmap_builder.padding_y * 2 >= current_height {
            if update_width {
                current_width *= 2; 
                update_width = false;
            } else { 
                current_height *= 2; 
                update_width = true;
            }

            continue;
        }
        
        let width_to_fit = current_width - bitmap_builder.padding_x * 2;
        let height_to_fit = current_height - bitmap_builder.padding_y * 2;
                
        if glyphs_fit_in(width_to_fit, height_to_fit, &glyphs, line_height, (bitmap_builder.spread * bitmap_builder.super_sampling_factor) as u32) {
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
fn glyphs_fit_in(width: u32, height: u32, glyphs: &Vec<PositionedGlyph<'_>>, line_height: u32, spread: u32) -> bool {
    let mut current_x: u32 = 0;
    let mut current_y: u32 = 0;

    let max_character_height = line_height + spread * 2;

    for (_, glyph) in glyphs.iter().enumerate() {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            let character_width = (bounding_box.max.x - bounding_box.min.x) as u32 + spread * 2;

            if current_x + character_width > width {
                // go to next line
                current_x = character_width;
                current_y += max_character_height;

                if current_y + max_character_height > height {
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
/// * `bitmap_builder` - 
/// * `image_buffer` - 
/// * `bitmap_characters` - 
/// * `line_height` - 
fn write_glyphs(
    glyphs: Vec<PositionedGlyph<'_>>, 
    bitmap_builder: &SdfBitmapBuilder,
    image_buffer: &mut GrayImage, 
    bitmap_characters: &mut HashMap<char, BitmapCharacter>,
    line_height: f32,
) {
    let mut current_x: u32 = bitmap_builder.padding_x;
    let mut current_y: u32 = bitmap_builder.padding_y;
    let spread = (bitmap_builder.spread * bitmap_builder.super_sampling_factor) as u32;

    for (i, glyph) in glyphs.iter().enumerate() {
        let bitmap_width = image_buffer.width();
        let bitmap_height = image_buffer.height();

        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            let character: char = bitmap_builder.characters.chars().nth(i).take().unwrap();
            let character_width = (bounding_box.max.x - bounding_box.min.x) as u32 + spread * 2;
            let character_height = (bounding_box.max.y - bounding_box.min.y) as u32 + spread * 2;

            if current_x + character_width >= bitmap_width - bitmap_builder.padding_x {
                // go to next line
                current_x = bitmap_builder.padding_x;
                current_y += line_height as u32 + spread * 2;

                if current_y >= bitmap_height - bitmap_builder.padding_y {
                    log::engine_err(format!("Failed to write glyphs to bitmap because it does not fit within the image"));
                    return;
                }
            }

            register_bitmap_character(
                bitmap_characters, 
                character, 
                bitmap_builder, 
                current_x, 
                current_y, 
                bitmap_width, 
                bitmap_height, 
                character_width, 
                line_height, 
                spread as f32
            );

            let glyph_buffer = create_glyph_binary_map(glyph, character_width as usize, character_height as usize, bitmap_builder);
            let border_pixels: Vec<(usize, usize)> = get_border_pixels(&glyph_buffer);

            for x in 0..glyph_buffer.len() {
                for y in 0..glyph_buffer[x].len() {
                    match get_pixel_value(&glyph_buffer, &border_pixels, x, y, spread as f32) {
                        Ok(v) => {
                            image_buffer.put_pixel(
                                x as u32 + current_x,
                                y as u32 + current_y + bounding_box.min.y as u32 - bitmap_builder.padding_y,
                                Luma([v]),
                            )
                        },
                        Err(_) => (),
                    }
                }
            }

            current_x += character_width;
        }
    }
}

fn is_border_pixel(glyph: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    if !glyph[x][y] {
        return false;
    }

    return !glyph[x - 1][y] || !glyph[x + 1][y] || !glyph[x][y - 1] || !glyph[x][y + 1];
}

fn register_bitmap_character(
    bitmap_characters: &mut HashMap<char, BitmapCharacter>, 
    character: char, 
    bitmap_builder: &SdfBitmapBuilder, 
    current_x: u32, 
    current_y: u32,
    bitmap_width: u32,
    bitmap_height: u32,
    character_width: u32,
    line_height: f32,
    spread: f32,
) {
    match bitmap_characters.entry(character) {
        std::collections::hash_map::Entry::Occupied(_) => {
            log::engine_warn(format!("Encountered duplicate character [{}] while writing glyphs for characters [{}] to bitmap", character, bitmap_builder.characters));
            return;
        },
        std::collections::hash_map::Entry::Vacant(entry) => {            
            entry.insert(BitmapCharacter {
                texture_start_x: (current_x as i32) as f32 / bitmap_width as f32,
                texture_end_x: (current_x + character_width) as f32 / bitmap_width as f32,
                texture_start_y: current_y as f32 / bitmap_height as f32,
                texture_end_y: (current_y as f32 + line_height + spread * 2.0) / bitmap_height as f32,
                width: character_width as f32 / line_height,
            });
        },
    }
}

/// Create a binary bitmap for the glyph with empty space for the spread.
fn create_glyph_binary_map(glyph: &PositionedGlyph<'_>, character_width: usize, character_height: usize, bitmap_builder: &SdfBitmapBuilder) -> Vec<Vec<bool>> {
    // A 2d vec that says where the pixels of the glyph are
    let mut glyph_pixels: Vec<Vec<bool>> = vec![vec![false; character_height]; character_width];

    let spread = (bitmap_builder.spread * bitmap_builder.super_sampling_factor) as u32;

    glyph.draw(|x, y, v| {
        if v < bitmap_builder.pixel_boundry {
            return;
        }

        let index_x = (x + spread) as usize;
        let index_y = (y + spread) as usize;

        glyph_pixels[index_x][index_y] = true;
    });

    glyph_pixels
}

fn get_distance_to_closest_border_pixel(border_pixels: &Vec<(usize, usize)>, target_x: usize, target_y: usize) -> f32 {
    let mut closest: f32 = f32::MAX;

    for coordinate in border_pixels.iter() {
        let distance = math::distance(target_x, target_y, coordinate.0, coordinate.1);
                
        if distance < closest {
            closest = distance;
        }
    }

    closest
}

fn get_border_pixels(glyph_pixels: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    
    for x in 0..glyph_pixels.len() {
        for y in 0..glyph_pixels[x].len() {
            if is_border_pixel(glyph_pixels, x, y) {
                result.push((x, y));
            }

        }
    }

    result
}

fn get_pixel_value(glyph_pixels: &Vec<Vec<bool>>,  border_pixels: &Vec<(usize, usize)>, x: usize, y: usize, spread: f32) -> Result<u8, ()> {
    if is_border_pixel(&glyph_pixels, x, y) {
        return Ok(127);
    } 

    let distance = get_distance_to_closest_border_pixel(&border_pixels, x, y);

    if distance > spread as f32 {
        if glyph_pixels[x][y] {
            return Ok(255);
        } else {
            return Err(());
        }
    }

    if glyph_pixels[x][y] {
        return Ok(127 + (distance / spread as f32 * 128.0) as u8);
    } else {
        return Ok(127 - (distance / spread as f32 * 127.0) as u8);
    }
    
}

use std::collections::HashMap;

use image::{DynamicImage, Rgba, RgbaImage, ImageBuffer};
use rusttype::PositionedGlyph;

use crate::lz_core_warn;

/// TODO - Spread glyphs over multiple lines. The advantages of this are:
///         1. Prevent texture size limits
///         2. More compact packing (glyphs can be put in any place there is space for it)
pub struct Bitmap {
    pub image: RgbaImage,
    pub characters: HashMap<char, BitmapCharacter>,
    pub line_height: f32,
}

impl Bitmap {
    pub fn new(font: &rusttype::Font<'static>, bitmap_builder: BitmapBuilder) -> Result<Bitmap, String> {
        if bitmap_builder.characters.len() == 0 {
            return Err("Failed to create bitmap: character set may not be empty".to_string());
        }

        let bitmap = Self::create(font, bitmap_builder).map_err(|err| {
            format!("Failed to create bitmap: {}", err)
        })?;
        Ok(bitmap)
    }

    /// save the bitmap image to a file
    pub fn save(&self, path: &String) -> Result<(), String> {
        let save_result = self.image.save(path).map_err(|err| {
            format!("Failed to save bitmap image: {}", err)
        });

        match save_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn create(font: &rusttype::Font<'static>, bitmap_builder: BitmapBuilder) -> Result<Self, String> {
        let scale = rusttype::Scale::uniform(bitmap_builder.font_size);
        let v_metrics = font.v_metrics(scale);
        let start_point = rusttype::point(bitmap_builder.padding as f32, bitmap_builder.padding as f32 + v_metrics.ascent);
        let glyphs: Vec<_> = font.layout(&bitmap_builder.characters, scale, start_point).collect();

        let total_glyphs_width = glyphs.last().take().unwrap().pixel_bounding_box().unwrap().max.x as u32;
        let line_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = DynamicImage::new_rgba8(
            total_glyphs_width + bitmap_builder.padding * 2, 
            line_height + bitmap_builder.padding * 2
        ).to_rgba8();
        let mut bitmap_characters: HashMap<char, BitmapCharacter> = HashMap::new();
        write_glyphs(glyphs, &bitmap_builder.characters, &mut image_buffer, &mut bitmap_characters, bitmap_builder.colour, bitmap_builder.padding, line_height as f32);

        let rgba_image = to_rgba_image(image_buffer)?;

        Ok(Self{ 
            image: rgba_image, 
            characters: bitmap_characters, 
            line_height: line_height as f32,
        })
    }

}

pub struct BitmapBuilder {
    colour: (u8, u8, u8),
    padding: u32,
    font_size: f32,
    characters: String,
}

impl BitmapBuilder {
    pub fn new() -> Self {
        Self {
            colour: (255, 255, 255),
            padding: 0,
            font_size: 25.0,
            characters: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!№;%:?*()_+-=.,/|\\\"'@#$€^&{}[]".to_string(),
        }
    }

    pub fn with_colour(mut self, colour: (u8, u8, u8)) -> Self {
        self.colour = colour;
        self
    }

    pub fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
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
}

#[derive(Debug)]
pub struct BitmapCharacter {
    pub texture_start_x: f32,
    pub texture_end_x: f32,
    pub texture_start_y: f32,
    pub texture_end_y: f32,
    pub width: f32, // relative to the lineheight of the font
}

fn write_glyphs(
    glyphs: Vec<PositionedGlyph<'_>>, 
    characters: &str, 
    image_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, 
    bitmap_characters: &mut HashMap<char, BitmapCharacter>,
    colour: (u8, u8, u8),
    padding: u32,
    line_height: f32,
) {
    for (i, glyph) in glyphs.iter().enumerate() {
        let bitmap_width = image_buffer.width() as f32;
        let bitmap_height = image_buffer.height() as f32;

        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            let character: char = characters.chars().nth(i).take().unwrap();

            match bitmap_characters.entry(character) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    lz_core_warn!("Encountered duplicate character [{}] while writing glyphs for characters [{}] to bitmap", character, characters);
                    continue;
                },
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(BitmapCharacter {
                        texture_start_x: (bounding_box.min.x + padding as i32) as f32 / bitmap_width,
                        texture_end_x: (bounding_box.max.x + padding as i32) as f32 / bitmap_width,
                        texture_start_y: 0.0,
                        texture_end_y: 1.0,
                        width: (bounding_box.max.x - bounding_box.min.x) as f32 / line_height
                    });
                },
            }

            glyph.draw(|x, y, v| {
                image_buffer.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                )
            });
        }
    }
}

fn to_rgba_image(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<RgbaImage, String> {
    let (width, height) = image_buffer.dimensions();
    let raw_pixels = image_buffer.into_raw();
    
    match RgbaImage::from_raw(width, height, raw_pixels) {
        Some(img) => Ok(img),
        None => Err(format!("Could not create RgbaImage")),
    }
}

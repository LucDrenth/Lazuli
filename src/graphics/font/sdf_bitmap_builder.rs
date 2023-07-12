use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use serde::Serialize;

use crate::lz_core_err;

use super::{bitmap::BitmapBuilder, Bitmap, sdf_bitmap::SdfBitmap, bitmap_cache::{SdfBitmapCache, self}};

#[derive(Serialize)]
pub struct SdfBitmapBuilder {
    pub padding_x: u32,
    pub padding_y: u32,
    pub font_size: f32,
    pub characters: String,
    pub pixel_boundry: f32, // a value between 0 and 1. If the alpha value of a glyph is equal or higher than this image, draw a pixel
    pub spread: u8, // the amount of padding (in pixels) each glyph from the binary image gets. Increase for better quality.
    pub super_sampling_factor: u8,
    pub cache: bool,
    pub vertex_shader_path: String,
    pub fragment_shader_path: String,
}

impl BitmapBuilder for SdfBitmapBuilder {
    fn build(&self, font: &rusttype::Font<'static>) -> Result<Box<dyn Bitmap>, String> {
        match SdfBitmap::new(font, &self) {
            Ok(bitmap) => return Ok(Box::new(bitmap)),
            Err(err) => Err(err),
        }
    }

    fn do_cache(&self) -> bool {
        self.cache
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

    fn cache_from_json(&self, data: String) -> Option<Box<dyn bitmap_cache::BitmapCache>> {
        let bitmap_cache: Result<SdfBitmapCache, serde_json::error::Error> = serde_json::from_str(&data);

        match bitmap_cache {
            Ok(cache) => return Some(Box::new(cache)),
            Err(err) => {
                // cache exists but is not valid
                lz_core_err!("Failed to read data from sdf bitmap cache: {}", err.to_string());
                return None;
            },
        }
    }

    fn vertex_shader_path(&self) -> &String {
        &self.vertex_shader_path
    }

    fn fragment_shader_path(&self) -> &String {
        &self.fragment_shader_path
    }

    
}

impl SdfBitmapBuilder {
    pub fn new() -> Self {
        Self {
            padding_x: 0,
            padding_y: 0,
            font_size: 25.0,
            characters: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!;%:?*()_+-=.,/|\\\"'@#$€^&{}[]".to_string(),
            pixel_boundry: 0.5,
            spread: 4,
            super_sampling_factor: 4,
            cache: true,
            vertex_shader_path: "./assets/shaders/text-ui.vert".to_string(),
            fragment_shader_path: "./assets/shaders/text-ui-sdf.frag".to_string(),
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

    // A value between 0 and 1. If the alpha value of a glyph is equal or higher than this image, draw a pixel
    pub fn with_pixel_boundry(mut self, pixel_boundry: f32) -> Self {
        self.pixel_boundry = pixel_boundry;
        self
    }

    pub fn with_spread(mut self, spread: u8) -> Self {
        self.spread = spread;
        self
    }

    pub fn with_super_sampling_factor(mut self, super_sampling_factor: u8) -> Self {
        self.super_sampling_factor = super_sampling_factor;
        self
    }

    pub fn with_cache(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    pub fn with_vertex_shader_path(mut self, path: String) -> Self {
        self.vertex_shader_path = path;
        self
    }

    pub fn with_fragment_shader_path(mut self, path: String) -> Self {
        self.fragment_shader_path = path;
        self
    }
}

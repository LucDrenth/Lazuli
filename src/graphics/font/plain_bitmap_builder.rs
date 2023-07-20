use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use serde::Serialize;

use crate::{lz_core_err, graphics::shader::ShaderBuilder};

use super::{bitmap::BitmapBuilder, Bitmap, plain_bitmap::PlainBitmap, bitmap_cache::PlainBitmapCache};

#[derive(Serialize)]
pub struct PlainBitmapBuilder {
    pub padding_x: u32, // padding arround the image
    pub padding_y: u32, // padding around the image
    pub font_size: f32,
    pub characters: String,
    pub cache: bool,
    pub super_sampling_factor: u8,
    pub glyph_padding_x: u32, // padding between the glyphs
    pub glyph_padding_y: u32, // padding between the glyphs
    pub font_file_path: String,
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

    fn default_shader_builder(&self) -> ShaderBuilder {
        ShaderBuilder::new()
            .with_vertex_shader_path("./assets/shaders/ui/text.vert".to_string())
            .with_fragment_shader_path("./assets/shaders/ui/text-plain.frag".to_string())
    }

    fn font_file_path(&self) -> &String {
        &self.font_file_path
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
            super_sampling_factor: 1,
            glyph_padding_x: 1, // setting this to a minimum of one prevents overlapping when downsampling
            glyph_padding_y: 1,
            font_file_path: "./assets/fonts/roboto.ttf".to_string(),
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

    pub fn with_super_sampling_factor(mut self, super_sampling_factor: u8) -> Self {
        self.super_sampling_factor = super_sampling_factor;
        self
    }

    pub fn with_glyph_padding_x(mut self, padding: u32) -> Self {
        self.glyph_padding_x = padding;
        self
    }

    pub fn with_glyph_padding_y(mut self, padding: u32) -> Self {
        self.glyph_padding_y = padding;
        self
    }

    pub fn with_font_file_path(mut self, path: String) -> Self {
        self.font_file_path = path;
        self
    }
}

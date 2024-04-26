use std::collections::HashMap;

use crate::graphics::{texture::ImageType, shader::ShaderBuilder};

use super::{BitmapCharacter, bitmap_cache::BitmapCache};

pub trait Bitmap {
    fn image(&self) -> &ImageType;
    fn save(&self, path: &String) -> Result<(), String>;
    fn characters(&self) -> &HashMap<char, BitmapCharacter>;
    fn line_height(&self) -> f32;
    fn spread(&self) -> u8;
    fn to_json_cache(&self) -> Result<String, String>;
}

pub trait BitmapBuilder {
    fn build(&self, font: &rusttype::Font<'static>) -> Result<Box<dyn Bitmap>, String>;
    fn get_hash(&self) -> Result<String, String>;
    fn cache_from_json(&self, data: String) -> Option<Box<dyn BitmapCache>>;
    fn do_cache(&self) -> bool;
    fn default_shader_builder(&self) -> ShaderBuilder;
    fn font_file_path(&self) -> &String;
}

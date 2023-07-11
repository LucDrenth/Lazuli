use std::collections::HashMap;

use crate::graphics::texture::ImageType;

use super::BitmapCharacter;

pub trait Bitmap {
    fn image(&self) -> &ImageType;
    fn save(&self, path: &String) -> Result<(), String>;
    fn characters(&self) -> &HashMap<char, BitmapCharacter>;
    fn line_height(&self) -> f32;
    fn spread(&self) -> u8; // TODO rename to something more general
}

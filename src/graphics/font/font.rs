use std::{fs::File, io::Read};

use crate::{lz_core_err, lz_core_warn, graphics::{texture::ImageType, shader::ShaderProgram, material::Material}};

use super::{BitmapCharacter, Bitmap, bitmap::BitmapBuilder, bitmap_cache};

pub struct Font {
    bitmap: Box<dyn Bitmap>,

    /// The width of the space (' ') character. the space is relative to the line height. So 0.5 is halve the line height. 
    pub space_size: f32,
    pub material: Material,
}

impl Font {
    pub fn new(path: String, bitmap_builder: impl BitmapBuilder) -> Result<Self, String> {
        match load_font(&path) {
            Ok(font) => {
                let bitmap = Self::get_bitmap(font, &path, &bitmap_builder)?;
            
                let program = ShaderProgram::new(bitmap_builder.vertex_shader_path(), bitmap_builder.fragment_shader_path())?;
                let mut material = Material::new(program);
                material.add_texture_from_image(bitmap.image());

                Ok(Self { 
                    bitmap,
                    space_size: 0.215,
                    material,
                })
            },
            Err(err) => {
                lz_core_err!("Failed to create font from path {}: {}", path, err);
                Err(err)
            },
        }
    }

    pub fn save_bitmap(&self, path: String) -> Result<(), String> {
        self.bitmap.save(&path)
    }

    pub fn image(&self) -> &ImageType {
        &self.bitmap.image()
    }

    pub fn get_bitmap_character(&self, character: char) -> Option<&BitmapCharacter> {
        self.bitmap.characters().get(&character)
    }

    pub fn line_height(&self) -> f32 {
        self.bitmap.line_height()
    }

    pub fn bitmap_spread(&self) -> u8 {
        self.bitmap.spread()
    }

    fn get_bitmap(font: rusttype::Font<'static>, path: &String, bitmap_builder: &impl BitmapBuilder) -> Result<Box<dyn Bitmap>, String> {
        if let Some(existing_bitmap) = bitmap_cache::load(&path, bitmap_builder) {
            return Ok(existing_bitmap)
        } else {
            let new_bitmap = bitmap_builder.build(&font)?;

            if bitmap_builder.do_cache() {
                match bitmap_cache::save(path, bitmap_builder, &new_bitmap) {
                    Ok(_) => (),
                    Err(err) => {
                        lz_core_warn!("Failed to save sdf font bitmap cache: {}", err);
                    },
                }
            }

            return Ok(new_bitmap);
        }
    }
}

pub fn load_font(path: &String) -> Result<rusttype::Font<'static>, String> {
    let mut file = File::open(path).map_err(|err| {
        format!("Failed to open file: {}", err.to_string())
    })?;

    let mut font_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut font_data).map_err(|err| {
        format!("Failed to read file: {}", err.to_string())
    })?;

    match rusttype::Font::try_from_vec(font_data) {
        Some(font) => Ok(font),
        None => Err(format!("Failed to create Font")),
    }
}

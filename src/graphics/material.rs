use std::path::Path;

use image::RgbaImage;

use super::{shader::ShaderProgram, texture::{Texture, TextureImage}};

pub struct Material {
    pub shader_program: ShaderProgram,
    textures: Vec<Texture>,
}

impl Material {
    pub fn new(shader_program: ShaderProgram) -> Self {
        Self {
            shader_program,
            textures: vec![],
        }
    }

    pub fn add_texture(&mut self, texture: Texture) {
        self.shader_program.set_uniform(
            format!("texture{}", self.textures.len()).as_str(), 
            self.textures.len() as i32
        );

        self.textures.push(texture);
    }

    pub fn add_texture_from_path(&mut self, path: &Path) {
        let texture = Texture::new();
        texture.load_from_path(path);

        self.add_texture(texture);
    }

    pub fn add_texture_from_image<T: Into<TextureImage>>(&mut self, img: T) {
        let texture = Texture::new();
        texture.load_from_image(img);
        
        self.add_texture(texture);
    }

    pub fn activate(&self) {
        self.shader_program.apply();
        
        for (index, texture) in self.textures.iter().enumerate() {
            texture.activate(index);
        }
    }
}

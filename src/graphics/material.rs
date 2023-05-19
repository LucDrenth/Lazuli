use std::path::Path;

use super::{shader::ShaderProgram, texture::Texture};

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

    pub fn add_texture(&mut self, path: &Path) {
        unsafe {
            let texture = Texture::new();
            texture.load(path);

            self.shader_program.set_uniform_int(
                format!("texture{}", self.textures.len()).as_str(), 
                self.textures.len().try_into().unwrap()
            );

            self.textures.push(texture);
        }
    }

    pub fn activate_materials(&self) {
        unsafe {
            for (index, texture) in self.textures.iter().enumerate() {
                texture.activate(index);
            }
        }
    }
}
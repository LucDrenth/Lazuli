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
        let texture = Texture::new();
        texture.load(path);

        self.shader_program.set_uniform(
            format!("texture{}", self.textures.len()).as_str(), 
            self.textures.len() as i32
        );

        self.textures.push(texture);
    }

    pub fn activate(&self) {
        for (index, texture) in self.textures.iter().enumerate() {
            texture.activate(index);
        }
    }
}

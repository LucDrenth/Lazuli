use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use super::ShaderProgram;

#[derive(Debug, Clone)]
pub struct ShaderBuilder {
    vertex_shader_path: String,
    fragment_shader_path: String,
}

impl ShaderBuilder {
    pub fn new(vertex_shader_path: impl Into<String>, fragment_shader_path: impl Into<String>) -> Self {
        ShaderBuilder {
            vertex_shader_path: vertex_shader_path.into(),
            fragment_shader_path: fragment_shader_path.into(),
        }
    }

    pub fn build(&self) -> Result<ShaderProgram, String> {
        ShaderProgram::new(&self.vertex_shader_path, &self.fragment_shader_path)
    }

    pub fn with_vertex_shader(mut self, path: impl Into<String>) -> Self {
        self.vertex_shader_path = path.into();
        self
    }

    pub fn with_fragment_shader(mut self, path: impl Into<String>) -> Self {
        self.fragment_shader_path = path.into();
        self
    }

    pub fn hash(&self) -> Result<u64, String> {
        let mut hasher = DefaultHasher::new();
        self.vertex_shader_path.hash(&mut hasher);
        self.fragment_shader_path.hash(&mut hasher);

        Ok(hasher.finish())
    }
}

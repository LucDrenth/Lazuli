use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use super::{program::GlShaderProgram, ShaderProgram};

pub trait ShaderBuilder {
    fn build(&self) -> Result<Box<dyn ShaderProgram>, String>;
    fn hash(&self) -> Result<u64, String>;
}

impl ShaderBuilder for GlShaderBuilder {
    fn build(&self) -> Result<Box<dyn ShaderProgram>, String> {
        let shader_program = GlShaderProgram::new(&self.vertex_shader_path, &self.fragment_shader_path)?;
        Ok(Box::new(shader_program))
    }

    fn hash(&self) -> Result<u64, String> {
        let mut hasher = DefaultHasher::new();
        self.vertex_shader_path.hash(&mut hasher);
        self.fragment_shader_path.hash(&mut hasher);

        Ok(hasher.finish())
    }
}

#[derive(Debug, Clone)]
pub struct GlShaderBuilder {
    vertex_shader_path: String,
    fragment_shader_path: String,
}

impl GlShaderBuilder {
    pub fn new(vertex_shader_path: impl Into<String>, fragment_shader_path: impl Into<String>) -> Self {
        Self {
            vertex_shader_path: vertex_shader_path.into(),
            fragment_shader_path: fragment_shader_path.into(),
        }
    }

    pub fn with_vertex_shader(mut self, path: impl Into<String>) -> Self {
        self.vertex_shader_path = path.into();
        self
    }

    pub fn with_fragment_shader(mut self, path: impl Into<String>) -> Self {
        self.fragment_shader_path = path.into();
        self
    }
}

use super::ShaderProgram;

pub struct ShaderBuilder {
    vertex_shader_path: String,
    fragment_shader_path: String,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        ShaderBuilder {
            vertex_shader_path: "".to_string(),
            fragment_shader_path: "".to_string(),
        }
    }

    pub fn build(&self) -> Result<ShaderProgram, String> {
        ShaderProgram::new(&self.vertex_shader_path, &self.fragment_shader_path)
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

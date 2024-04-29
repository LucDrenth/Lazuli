use super::{program_mock::MockShaderProgram, ShaderBuilder, ShaderProgram};

pub struct MockShaderBuilder {
    pub hash: u64,
}
impl ShaderBuilder for MockShaderBuilder {
    fn build(&self) -> Result<Box<dyn ShaderProgram>, String> {
        let shader_program = MockShaderProgram{};
        Ok(Box::new(shader_program))
    }

    fn hash(&self) -> Result<u64, String> {
        Ok(self.hash)
    }
}

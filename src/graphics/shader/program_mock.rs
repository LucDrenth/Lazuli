use super::ShaderProgram;

pub struct MockShaderProgram {

}

impl ShaderProgram for MockShaderProgram {
    fn apply(&self) {}

    fn set_uniform(&self, _name: &str, _value: &super::UniformValue) {}

    fn get_uniform_location(&self, _name: &str) -> i32 {
        1
    }

    fn get_attribute_location(&self, _attribute: &str) -> Result<gl::types::GLuint, String> {
        Ok(2)
    }
}

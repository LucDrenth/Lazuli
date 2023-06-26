use crate::graphics::shader::ShaderProgram;

type Position = [f32; 3];
type Color = [f32; 3];
type TextureCoordinates = [f32; 2];

pub struct VertexColored(pub Position, pub Color);
pub struct VertexTextured(pub Position, pub TextureCoordinates);

pub trait Shape {
    fn draw(&self, program: &ShaderProgram);
}

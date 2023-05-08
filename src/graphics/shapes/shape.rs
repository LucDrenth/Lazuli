type Position = [f32; 3];
type Color = [f32; 3];
type TextureCoordinates = [f32; 2];

#[repr(C, packed)]
pub struct VertexColored(pub Position, pub Color);
pub struct VertexTextured(pub Position, pub TextureCoordinates);

pub trait Shape {
    unsafe fn draw(&self);
}

type Position = [f32; 3];
type Color = [f32; 3];

#[repr(C, packed)]
pub struct Vertex(pub Position, pub Color);

pub trait Shape {
    unsafe fn draw(&self);
}

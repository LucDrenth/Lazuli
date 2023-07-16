mod shape;
mod triangle;
mod rectangle;
mod cube;

pub use shape::Shape;
pub use triangle::Triangle;
pub use rectangle::Rectangle;
pub use cube::Cube;

pub use rectangle::INDICES as RECTANGLE_INDICES;
pub use shape::VertexColored;

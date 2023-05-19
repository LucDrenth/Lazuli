pub mod renderer;
pub mod shader;
mod shapes;

pub mod window;
pub mod texture;
pub mod material;
pub mod scene;

pub use shapes::Triangle;
pub use shapes::Rectangle;
pub use renderer::mesh_renderer;
pub use window::Window;

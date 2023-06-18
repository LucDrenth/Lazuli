mod shapes;
mod transform;
mod camera;

pub mod renderer;
pub mod shader;
pub mod window;
pub mod texture;
pub mod material;
pub mod scene;

pub use shapes::Triangle;
pub use shapes::Rectangle;
pub use renderer::mesh_renderer;
pub use window::Window;
pub use transform::Transform;
pub use camera::Camera;
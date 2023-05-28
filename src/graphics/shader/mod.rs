mod program;
mod shader;
mod uniform;

pub use shader::Shader;
pub use program::ShaderProgram;

pub use shader::PATH_COLORED_VERT;
pub use shader::PATH_COLORED_FRAG;
pub use shader::PATH_TEXTURED_VERT;
pub use shader::PATH_TEXTURED_FRAG;
pub use shader::PATH_MOVING_TRIANGLE_VERT;
pub use shader::PATH_MOVING_TRIANGLE_FRAG;

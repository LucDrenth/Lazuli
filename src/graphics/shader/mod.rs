mod program;
mod shader;
mod uniform;
mod shader_builder;

pub use shader::Shader;
pub use program::ShaderProgram;
pub use shader_builder::ShaderBuilder;

pub use shader::PATH_COLORED_VERT;
pub use shader::PATH_COLORED_FRAG;
pub use shader::PATH_TEXTURED_VERT;
pub use shader::PATH_TEXTURED_FRAG;
pub use shader::PATH_TEXTURE_MASK_VERT;
pub use shader::PATH_TEXTURE_MASK_FRAG;
pub use shader::PATH_HELLO_TRANFORM_VERT;
pub use shader::PATH_MOVING_TRIANGLE_VERT;
pub use shader::PATH_MOVING_TRIANGLE_FRAG;

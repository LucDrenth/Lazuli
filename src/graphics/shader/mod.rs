mod program;
mod shader;
mod uniform;
mod shader_builder;
mod custom_shader_values;

pub use shader::Shader;
pub use program::ShaderProgram;
pub use uniform::UniformValue;
pub use shader_builder::ShaderBuilder;
pub use shader_builder::GlShaderBuilder;
pub use custom_shader_values::CustomShaderValues;

pub use shader::PATH_COLORED_VERT;
pub use shader::PATH_COLORED_FRAG;
pub use shader::PATH_TEXTURED_VERT;
pub use shader::PATH_TEXTURED_FRAG;
pub use shader::PATH_TEXTURE_MASK_VERT;
pub use shader::PATH_TEXTURE_MASK_FRAG;
pub use shader::PATH_HELLO_TRANFORM_VERT;
pub use shader::PATH_MOVING_TRIANGLE_VERT;
pub use shader::PATH_MOVING_TRIANGLE_FRAG;

pub mod shader_builder_mock;
pub mod shader_mock;
pub mod program_mock;

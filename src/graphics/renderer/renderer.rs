use crate::{graphics::{shader::{ShaderProgram, Shader}, shapes::Triangle}};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec3 position;
in vec3 color;
out vec3 vertexColor;
void main() {
    gl_Position = vec4(position, 1.0);
    vertexColor = color;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
in vec3 vertexColor;
void main() {
    FragColor = vec4(vertexColor, 1.0);
}
"#;

pub struct Renderer {
    triangle: Triangle
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        unsafe {
            // Create shaders
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let triangle = Triangle::create(program);

            // TODO put this in a macro and use it at more places
            // error handling
            let mut err = gl::GetError();
            while err != gl::NO_ERROR {
                print!("gl error: {}", err);
                err = gl::GetError();
            }

            let result = Self { triangle };

            Ok(result)
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.45, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.triangle.draw()
        }
    }
}

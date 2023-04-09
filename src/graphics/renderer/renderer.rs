use crate::{graphics::shader::{ShaderProgram, Shader}, set_attribute};

use super::buffer::{Buffer, Vao};

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

type Position = [f32; 3];
type Color = [f32; 3];

#[repr(C, packed)]
pub struct Vertex(pub Position, pub Color);

const TRIANGLE_VERTICES: [Vertex; 3] = [
    Vertex([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5, 0.0], [0.0, 1.0, 0.0]),
    Vertex([0.0,   0.5, 0.0], [0.0, 0.0, 1.0])
];

pub struct Renderer {
    program: ShaderProgram,
    vertex_array: Vao,
    _vertex_buffer: Buffer, // For now, we Save this in here so it won't get destroyed and cleaned up
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        unsafe {
            // Create shaders
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;


            // Create triangle
            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&TRIANGLE_VERTICES, gl::STATIC_DRAW);

            let vertex_array = Vao::new();

            let position_attribute = program.get_attribute_location("position")
                .expect("Could not get position attribute");
            set_attribute!(vertex_array, position_attribute, Vertex::0);

            let color_attribute = program.get_attribute_location("color")
                .expect("Could not get color attribute");
            set_attribute!(vertex_array, color_attribute, Vertex::1);


            // TODO put this in a macro and use it at more places
            // error handling
            let mut err = gl::GetError();
            while err != gl::NO_ERROR {
                print!("gl error: {}", err);
                err = gl::GetError();
            }



            Ok(Self { 
                program, 
                vertex_array, 
                _vertex_buffer: vertex_buffer
             })
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.45, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.program.apply();
            self.vertex_array.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

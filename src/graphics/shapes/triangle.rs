use crate::{graphics::{shader::ShaderProgram, renderer::buffer::{Vao, Buffer}}, set_attribute};

type Position = [f32; 3];
type Color = [f32; 3];

#[repr(C, packed)]
pub struct Vertex(pub Position, pub Color);

const TRIANGLE_VERTICES: [Vertex; 3] = [
    Vertex([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5, 0.0], [0.0, 1.0, 0.0]),
    Vertex([0.0,   0.5, 0.0], [0.0, 0.0, 1.0])
];

pub struct Triangle {
    program: ShaderProgram,
    vertex_array: Vao,
    _vertex_buffer: Buffer,
}

impl Triangle {
    pub unsafe fn create(program: ShaderProgram) -> Self {
        let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&TRIANGLE_VERTICES, gl::STATIC_DRAW);

            let vertex_array = Vao::new();

            let position_attribute = program.get_attribute_location("position")
                .expect("Could not get position attribute");
            set_attribute!(vertex_array, position_attribute, Vertex::0);

            let color_attribute = program.get_attribute_location("color")
                .expect("Could not get color attribute");
            set_attribute!(vertex_array, color_attribute, Vertex::1);

        Self { 
            program, 
            vertex_array, 
            _vertex_buffer: vertex_buffer
         }
    }

    pub unsafe fn draw(&self) {
        self.program.apply();
        self.vertex_array.bind();
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

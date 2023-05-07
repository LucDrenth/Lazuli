use crate::{graphics::{shader::ShaderProgram, renderer::buffer::{Vao, Buffer}}, set_attribute, error::opengl};

use super::{Shape, shape::Vertex};

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
    pub unsafe fn new(program: ShaderProgram) -> Self {
        let mut vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
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
}

impl Shape for Triangle {
    unsafe fn draw(&self) {
        self.program.apply();
        self.vertex_array.bind();
        gl::DrawArrays(gl::TRIANGLES, 0, self._vertex_buffer.data_size as i32);

        opengl::gl_check_errors();
    }
}

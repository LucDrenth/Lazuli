use crate::{graphics::{shader::ShaderProgram, renderer::buffer::{Vao, Buffer}}, set_attribute, error::opengl};

use super::{Shape, shape::VertexColored};

const TRIANGLE_VERTICES: [VertexColored; 3] = [
    VertexColored([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
    VertexColored([0.5,  -0.5, 0.0], [0.0, 1.0, 0.0]),
    VertexColored([0.0,   0.5, 0.0], [0.0, 0.0, 1.0])
];

pub struct Triangle {
    vao: Vao,
    vbo: Buffer,
}

impl Triangle {
    pub unsafe fn new(program: &ShaderProgram) -> Self {
        let mut vbo = Buffer::new(gl::ARRAY_BUFFER);
        vbo.set_data(&TRIANGLE_VERTICES, gl::STATIC_DRAW);

        let vao = Vao::new();

        let position_attribute = program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, VertexColored::0);

        let color_attribute = program.get_attribute_location("color")
            .expect("Could not get color attribute");
        set_attribute!(vao, color_attribute, VertexColored::1);

        Self { 
            vao, 
            vbo
         }
    }
}

impl Shape for Triangle {
    unsafe fn draw(&self, program: &ShaderProgram) {
        program.apply();
        self.vao.bind();
        gl::DrawArrays(gl::TRIANGLES, 0, self.vbo.data_size as i32);

        opengl::gl_check_errors();
    }
}

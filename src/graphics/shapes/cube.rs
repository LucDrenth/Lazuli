use crate::{graphics::{renderer::buffer::{Vao, Buffer}, shader::ShaderProgram}, error::opengl, set_attribute};

use super::{shape::VertexColored, Shape};

const COLORED_CUBE_VERTICES: [VertexColored; 24] = [
    // Front face (red)
    VertexColored([-0.5, -0.5, 0.5], [1.0, 0.0, 0.0]),   // bottom left
    VertexColored([0.5, -0.5, 0.5], [1.0, 0.0, 0.0]),    // bottom right
    VertexColored([0.5, 0.5, 0.5], [1.0, 0.0, 0.0]),     // top right
    VertexColored([-0.5, 0.5, 0.5], [1.0, 0.0, 0.0]),    // top left

    // Back face (green)
    VertexColored([-0.5, -0.5, -0.5], [0.0, 1.0, 0.0]),  // bottom left
    VertexColored([0.5, -0.5, -0.5], [0.0, 1.0, 0.0]),   // bottom right
    VertexColored([0.5, 0.5, -0.5], [0.0, 1.0, 0.0]),    // top right
    VertexColored([-0.5, 0.5, -0.5], [0.0, 1.0, 0.0]),   // top left

    // Right face (blue)
    VertexColored([0.5, -0.5, -0.5], [0.0, 0.0, 1.0]),   // bottom left
    VertexColored([0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),    // bottom right
    VertexColored([0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),     // top right
    VertexColored([0.5, 0.5, -0.5], [0.0, 0.0, 1.0]),    // top left

    // Left face (yellow)
    VertexColored([-0.5, -0.5, -0.5], [1.0, 1.0, 0.0]),  // bottom left
    VertexColored([-0.5, -0.5, 0.5], [1.0, 1.0, 0.0]),   // bottom right
    VertexColored([-0.5, 0.5, 0.5], [1.0, 1.0, 0.0]),    // top right
    VertexColored([-0.5, 0.5, -0.5], [1.0, 1.0, 0.0]),   // top left

    // Top face (cyan)
    VertexColored([-0.5, 0.5, -0.5], [0.0, 1.0, 1.0]),   // bottom left
    VertexColored([0.5, 0.5, -0.5], [0.0, 1.0, 1.0]),    // bottom right
    VertexColored([0.5, 0.5, 0.5], [0.0, 1.0, 1.0]),     // top right
    VertexColored([-0.5, 0.5, 0.5], [0.0, 1.0, 1.0]),    // top left

    // Bottom face (magenta)
    VertexColored([-0.5, -0.5, -0.5], [1.0, 0.0, 1.0]),  // bottom left
    VertexColored([0.5, -0.5, -0.5], [1.0, 0.0, 1.0]),   // bottom right
    VertexColored([0.5, -0.5, 0.5], [1.0, 0.0, 1.0]),    // top right
    VertexColored([-0.5, -0.5, 0.5], [1.0, 0.0, 1.0]),   // top left
];

const CUBE_INDICES: [i32; 36] = [
    0, 1, 2, 2, 3, 0,           // Front face
    4, 5, 6, 6, 7, 4,           // Back face
    8, 9, 10, 10, 11, 8,        // Right face
    12, 13, 14, 14, 15, 12,     // Left face
    16, 17, 18, 18, 19, 16,     // Top face
    20, 21, 22, 22, 23, 20,     // Bottom face
];

pub struct Cube {
    vao: Vao,
    _vbo: Buffer,
    ebo: Buffer,
}

impl Cube {
    pub fn new_colored(program: &Box<dyn ShaderProgram>) -> Self {
        let vao = Vao::new();
        vao.bind();
        
        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&COLORED_CUBE_VERTICES, gl::STATIC_DRAW);

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&CUBE_INDICES, gl::STATIC_DRAW);

        let position_attribute = program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, VertexColored::0);

        let color_attribute = program.get_attribute_location("color")
            .expect("Could not get color attribute");
        set_attribute!(vao, color_attribute, VertexColored::1);

        Self { 
            vao, 
            _vbo: vbo,
            ebo
        }
    }
}

impl Shape for Cube {
    fn draw(&self, program: &Box<dyn ShaderProgram>) {
        program.apply();
        self.vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.ebo.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());
        }

        opengl::gl_check_errors();
    }
}

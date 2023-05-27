use crate::{graphics::{shader::ShaderProgram, renderer::buffer::{Vao, Buffer}}, set_attribute, error::opengl};

use super::{Shape, shape::{VertexColored, VertexTextured}};

const COLORED_RECTANGLE_VERTICES: [VertexColored; 4] = [
    VertexColored([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]), // bottom left
    VertexColored([ 0.5, -0.5, 0.0], [1.0, 1.0, 0.5]), // bottom right
    VertexColored([ 0.5,  0.5, 0.0], [0.5, 0.5, 1.0]), // top right
    VertexColored([-0.5,  0.5, 0.0], [0.0, 0.0, 0.0]) // top left
];

const TEXTURED_RECTANGLE_VERTICES: [VertexTextured; 4] = [
    VertexTextured([-0.5, -0.5, 0.0], [0.0, 1.0]), // bottom left
    VertexTextured([ 0.5, -0.5, 0.0], [1.0, 1.0]), // bottom right
    VertexTextured([ 0.5,  0.5, 0.0], [1.0, 0.0]), // top right
    VertexTextured([-0.5,  0.5, 0.0], [0.0, 0.0]) // top left
];

const INDICES: [i32; 6] = [
    0, 1, 2,
    2, 3, 0
];

pub struct Rectangle {
    vao: Vao,
    _vbo: Buffer,
    ebo: Buffer,
}

impl Rectangle {    
    pub unsafe fn new_colored(program: &ShaderProgram) -> Self {
        let vao = Vao::new();
        vao.bind();
        
        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&COLORED_RECTANGLE_VERTICES, gl::STATIC_DRAW);

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&INDICES, gl::STATIC_DRAW);

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

    pub unsafe fn new_textured(program: &ShaderProgram) -> Self {
        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&TEXTURED_RECTANGLE_VERTICES, gl::STATIC_DRAW);

        let vao = Vao::new();
        vao.bind();

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&INDICES, gl::STATIC_DRAW);

        let position_attribute = program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, VertexTextured::0);

        let texture_coordinates_attribute = program.get_attribute_location("vertexTextureCoordinates")
            .expect("Could not get vertexTextureCoordinates attribute");
        set_attribute!(vao, texture_coordinates_attribute, VertexTextured::1);

        Self { 
            vao, 
            _vbo: vbo,
            ebo,
         }
    }
}

impl Shape for Rectangle {
    unsafe fn draw(&self, program: &ShaderProgram) {
        program.apply();
        self.vao.bind();
        gl::DrawElements(gl::TRIANGLES, self.ebo.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());

        opengl::gl_check_errors();
    }
}

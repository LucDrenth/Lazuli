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
    program: ShaderProgram,
    vertex_array: Vao,
    _vertex_buffer: Buffer,
    _index_buffer: Buffer,
}

impl Rectangle {    
    pub unsafe fn new_colored(program: ShaderProgram) -> Self {
        let vertex_array = Vao::new();
        vertex_array.bind();
        
        let mut vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        vertex_buffer.set_data(&COLORED_RECTANGLE_VERTICES, gl::STATIC_DRAW);

        let mut index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
        index_buffer.set_data(&INDICES, gl::STATIC_DRAW);

        let position_attribute = program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vertex_array, position_attribute, VertexColored::0);

        let color_attribute = program.get_attribute_location("color")
            .expect("Could not get color attribute");
        set_attribute!(vertex_array, color_attribute, VertexColored::1);

        Self { 
            program, 
            vertex_array, 
            _vertex_buffer: vertex_buffer,
            _index_buffer: index_buffer,
         }
    }

    pub unsafe fn new_textured(program: ShaderProgram) -> Self {
        let vertex_array = Vao::new();
        vertex_array.bind();
        
        let mut vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        vertex_buffer.set_data(&TEXTURED_RECTANGLE_VERTICES, gl::STATIC_DRAW);

        let mut index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
        index_buffer.set_data(&INDICES, gl::STATIC_DRAW);

        let position_attribute = program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vertex_array, position_attribute, VertexTextured::0);

        let texture_coordinates_attribute = program.get_attribute_location("vertexTextureCoordinates")
            .expect("Could not get vertexTextureCoordinates attribute");
        set_attribute!(vertex_array, texture_coordinates_attribute, VertexTextured::1);

        Self { 
            program, 
            vertex_array, 
            _vertex_buffer: vertex_buffer,
            _index_buffer: index_buffer,
         }
    }
}

impl Shape for Rectangle {
    unsafe fn draw(&self) {
        self.program.apply();
        self.vertex_array.bind();
        gl::DrawElements(gl::TRIANGLES, self._index_buffer.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());

        opengl::gl_check_errors();
    }
}

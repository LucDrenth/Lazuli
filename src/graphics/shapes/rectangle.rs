use crate::{graphics::{shader::ShaderProgram, renderer::buffer::{Vao, Buffer}}, set_attribute};

use super::{Shape, shape::Vertex};

const RECTANGLE_VERTICES: [Vertex; 4] = [
    Vertex([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]), // bottom left
    Vertex([ 0.5, -0.5, 0.0], [1.0, 1.0, 0.5]), // bottom right
    Vertex([ 0.5,  0.5, 0.0], [0.5, 0.5, 1.0]), // top right
    Vertex([-0.5,  0.5, 0.0], [0.0, 0.0, 0.0]) // top left
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
    pub unsafe fn new(program: ShaderProgram) -> Self {
        let vertex_array = Vao::new();
        vertex_array.bind();
        
        let mut vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        vertex_buffer.set_data(&RECTANGLE_VERTICES, gl::STATIC_DRAW);

        let mut index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
        index_buffer.set_data(&INDICES, gl::STATIC_DRAW);

        let position_attribute = program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vertex_array, position_attribute, Vertex::0);

        let color_attribute = program.get_attribute_location("color")
            .expect("Could not get color attribute");
        set_attribute!(vertex_array, color_attribute, Vertex::1);

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
    }
}

use crate::{graphics::{renderer::buffer::{Vao, Buffer}, font::BitmapCharacter, shader::ShaderProgram}, error::opengl, set_attribute, lz_core_info};

type Position = [f32; 3];
type TextureCoordinates = [f32; 2];
type GlyphQuad = [GlyphVertex; 4];

pub struct GlyphVertex(pub Position, pub TextureCoordinates);

const INDICES: [i32; 6] = [
    0, 1, 2,
    2, 3, 0
];

pub struct Glyph {
    vao: Vao,
    _vbo: Buffer,
    ebo: Buffer,
}

impl Glyph {
    pub fn new(bitmap_character: &BitmapCharacter, start_x: f32, end_x: f32, program: &ShaderProgram) -> Self {
        let vertices: [GlyphVertex; 4] = [
            GlyphVertex([start_x, -1.0, 0.0], [bitmap_character.start_x, 1.0]), // bottom left
            GlyphVertex([end_x,   -1.0, 0.0], [bitmap_character.end_x,   1.0]), // bottom right
            GlyphVertex([end_x,    1.0, 0.0], [bitmap_character.end_x,   0.0]), // top right
            GlyphVertex([start_x,  1.0, 0.0], [bitmap_character.start_x, 0.0]) // top left
        ];

        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&vertices, gl::STATIC_DRAW);

        let vao = Vao::new();
        vao.bind();

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&INDICES, gl::STATIC_DRAW);

        let position_attribute = program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, GlyphVertex::0);

        let texture_coordinates_attribute = program.get_attribute_location("vertexTextureCoordinates")
            .expect("Could not get vertexTextureCoordinates attribute");
        set_attribute!(vao, texture_coordinates_attribute, GlyphVertex::1);

        Self {
            vao,
            _vbo: vbo,
            ebo,
        }
    }

    /// Program has to be applied before calling this function
    pub fn draw(&self) {
        self.vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.ebo.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());
        }

        opengl::gl_check_errors();
    }
}

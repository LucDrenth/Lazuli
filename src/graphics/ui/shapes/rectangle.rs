use glam::Vec2;

use crate::{graphics::{renderer::buffer::{Buffer, Vao}, shader::ShaderBuilder, ui::{ui_element::UiElement, Interface}, material::Material}, set_attribute, error::opengl};
use crate::graphics::shapes::RECTANGLE_INDICES;

type Positon = [f32; 3];
pub struct Vertex(Positon);

pub struct Rectangle {
    vao: Vao,
    _vbo: Buffer,
    ebo: Buffer,
    material_id: String,
    pub position: Vec2,
    color: (u8, u8, u8)
}

impl UiElement for Rectangle {
    fn material<'a>(&'a self, interface: &'a crate::graphics::ui::Interface) -> Option<&crate::graphics::material::Material> {
        interface.get_material(&self.material_id)
    }

    fn draw(&self, material: &crate::graphics::material::Material) {
        material.shader_program.apply();
        self.vao.bind();

        material.shader_program.set_uniform("color", (
            (self.color.0 as f32 / 255.0),
            (self.color.1 as f32 / 255.0),
            (self.color.2 as f32 / 255.0),
        ));
        material.shader_program.set_uniform("worldPosition", self.position_for_shader());

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.ebo.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());
        }

        opengl::gl_check_errors();
    }
}

impl Rectangle {
    pub fn new(builder: RectangleBuilder, interface: &mut Interface) -> Result<Self, String> {
        let shader_builder = match builder.shader_builder {
            Some(custom_shader_builder) => custom_shader_builder,
            None => Self::default_shader_builder(),
        };

        let material_id = shader_builder.hash().unwrap();

        match interface.get_material(&material_id) {
            Some(_) => (),
            None => {
                if !interface.add_material(
                    Material::new(shader_builder.build().unwrap()),
                    material_id.clone(),
                ) {
                    return Err("failed to rectangle material shader to interface".to_string());
                }
            },
        }

        let width = 100.0;
        let height = 40.0;

        let vertices: [Vertex; 4] = [
            Vertex([-width, -height, 0.0]), // bottom left
            Vertex([width, -height, 0.0]), // bottom right
            Vertex([width, height, 0.0]), // top right
            Vertex([-width, height, 0.0])  // top left
        ];

        let vao = Vao::new();
        vao.bind();
        
        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&vertices, gl::STATIC_DRAW);

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&RECTANGLE_INDICES, gl::STATIC_DRAW);

        let position_attribute = interface.get_material(&material_id).unwrap().shader_program.get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, Vertex::0);

        Ok(Self { 
            vao, 
            _vbo: vbo,
            ebo,
            material_id,
            color: builder.color,
            position: Vec2 { x: builder.position_x, y: builder.position_y }
        })
    }

    pub fn position_for_shader(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    pub fn default_shader_builder() -> ShaderBuilder {
        ShaderBuilder::new()
            .with_vertex_shader_path("./assets/shaders/ui/rectangle.vert".to_string())
            .with_fragment_shader_path("./assets/shaders/ui/rectangle.frag".to_string())
    }
}

pub struct RectangleBuilder {
    color: (u8, u8, u8),
    position_x: f32,
    position_y: f32,
    shader_builder: Option<ShaderBuilder>
}

impl RectangleBuilder {
    pub fn new() -> Self {
        Self {
            color: (126, 126, 126), // gray
            shader_builder: None,
            position_x: 0.0,
            position_y: 0.0,
        }
    }

    pub fn with_color(mut self, color: (u8, u8, u8)) -> Self {
        self.color = color;
        self
    }

    pub fn with_position_x(mut self, position_x: f32) -> Self {
        self.position_x = position_x;
        self
    }

    pub fn with_position_y(mut self, position_y: f32) -> Self {
        self.position_y = position_y;
        self
    }

    pub fn with_shader_builder(mut self, shader_builder: ShaderBuilder) -> Self {
        self.shader_builder = Some(shader_builder);
        self
    }
}

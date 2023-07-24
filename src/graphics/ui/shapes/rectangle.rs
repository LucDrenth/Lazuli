use glam::Vec2;

use crate::{graphics::{renderer::buffer::{Buffer, Vao}, shader::ShaderBuilder, ui::{ui_element::UiElement, interface::{is_valid_z_index, map_z_index_for_shader}}}, set_attribute, error::opengl, asset_registry::AssetRegistry, lz_core_warn};
use crate::graphics::shapes::RECTANGLE_INDICES;

type Positon = [f32; 3];
pub struct Vertex(Positon);

pub struct Rectangle {
    vao: Vao,
    _vbo: Buffer,
    ebo: Buffer,
    material_id: u32,
    pub position: Vec2,
    pub z_index: f32,
    color: (u8, u8, u8)
}

impl UiElement for Rectangle {
    fn draw(&self, asset_registry: &mut AssetRegistry) {
        let shader_id = asset_registry.get_material_by_id(self.material_id).unwrap().shader_id;
        let shader = asset_registry.get_shader_by_id(shader_id).unwrap();

        shader.apply();
        self.vao.bind();

        shader.set_uniform("color", (
            (self.color.0 as f32 / 255.0),
            (self.color.1 as f32 / 255.0),
            (self.color.2 as f32 / 255.0),
        ));
        shader.set_uniform("zIndex", map_z_index_for_shader(self.z_index));
        shader.set_uniform("worldPosition", self.position_for_shader());

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.ebo.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());
        }

        opengl::gl_check_errors();
    }

    fn material_id(&self) -> u32 {
        self.material_id
    }

    fn get_z_index(&self) -> f32 {
        self.z_index
    }

    fn type_name(&self) -> &str {
        "rectangle"
    }
}

impl Rectangle {
    pub fn new(builder: RectangleBuilder, asset_registry: &mut AssetRegistry) -> Result<Self, String> {
        let shader_builder = match builder.shader_builder {
            Some(custom_shader_builder) => custom_shader_builder,
            None => Self::default_shader_builder(),
        };

        let shader_id = asset_registry.load_shader(shader_builder)?;
        let material_id = asset_registry.load_material(shader_id)?;

        let vertices: [Vertex; 4] = [
            Vertex([-builder.width, -builder.height, 0.0]), // bottom left
            Vertex([builder.width, -builder.height, 0.0]), // bottom right
            Vertex([builder.width, builder.height, 0.0]), // top right
            Vertex([-builder.width, builder.height, 0.0])  // top left
        ];

        let vao = Vao::new();
        vao.bind();
        
        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&vertices, gl::STATIC_DRAW);

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&RECTANGLE_INDICES, gl::STATIC_DRAW);

        let position_attribute = asset_registry.get_shader_by_id(shader_id).unwrap().get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, Vertex::0);

        Ok(Self { 
            vao, 
            _vbo: vbo,
            ebo,
            material_id,
            color: builder.color,
            position: Vec2 { x: builder.position_x, y: builder.position_y },
            z_index: builder.z_index,
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
    shader_builder: Option<ShaderBuilder>,
    width: f32,
    height: f32,
    z_index: f32,
}

impl RectangleBuilder {
    pub fn new() -> Self {
        Self {
            color: (126, 126, 126), // gray
            shader_builder: None,
            position_x: 0.0,
            position_y: 0.0,
            width: 100.0,
            height: 40.0,
            z_index: 1.0,
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

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        if is_valid_z_index(z_index) {
            self.z_index = z_index;
        } else {
            lz_core_warn!("did not set RectangleBuilder z_index {} because it's not a valid z-index", z_index);
        }

        self
    }
}

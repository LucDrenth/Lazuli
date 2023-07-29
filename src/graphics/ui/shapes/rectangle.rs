use glam::Vec2;

use crate::{graphics::{renderer::buffer::{Buffer, Vao}, shader::ShaderBuilder, ui::{interface::{is_valid_z_index, map_z_index_for_shader}, element::{world_element_data::WorldElementData, ui_element::UiElement}, Position}, material::Material}, set_attribute, error::opengl, asset_registry::{AssetRegistry, AssetId}, log};
use crate::graphics::shapes::RECTANGLE_INDICES;

type VertexPosition = [f32; 2];
pub struct Vertex(VertexPosition);

pub struct Rectangle {
    vao: Vao,
    _vbo: Buffer,
    ebo: Buffer,
    material_id: AssetId<Material>,
    world_data: WorldElementData,
    color: (u8, u8, u8),
}

impl UiElement for Rectangle {
    fn draw(&self, asset_registry: &mut AssetRegistry) {
        let shader_id = asset_registry.get_material_by_id(&self.material_id).unwrap().shader_id.duplicate();
        let shader = asset_registry.get_shader_by_id(&shader_id).unwrap();

        shader.apply();
        self.vao.bind();

        shader.set_uniform("color", (
            (self.color.0 as f32 / 255.0),
            (self.color.1 as f32 / 255.0),
            (self.color.2 as f32 / 255.0),
        ));
        shader.set_uniform("scale", (self.world_data.scale.x, self.world_data.scale.y));
        shader.set_uniform("zIndex", map_z_index_for_shader(self.world_data.z_index()));
        shader.set_uniform("worldPosition", self.world_data.shader_coordinates());

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.ebo.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());
        }

        opengl::gl_check_errors();
    }

    fn material_id(&self) -> &AssetId<Material> {
        &self.material_id
    }

    fn type_name(&self) -> &str {
        "rectangle"
    }

    fn world_data(&self) -> &WorldElementData {
        &self.world_data
    }

    fn center_at(&mut self, element_to_center_on: &WorldElementData, window_size: &Vec2) {
        self.world_data.center_at(element_to_center_on, window_size);
    }

    fn handle_window_resize(&mut self, new_window_size: &Vec2) {
        self.world_data.handle_window_resize(new_window_size);
    }

    fn get_scale(&self) -> Vec2 { self.world_data.scale }
    fn set_scale(&mut self, new_scale: Vec2) { self.world_data.scale = new_scale; }
    fn get_size(&self) -> Vec2 { self.world_data.size().clone() }
    fn get_screen_position(&self) -> Vec2 { self.world_data.final_coordinates().clone() }
    
    fn set_text(&mut self, text: &String, _asset_registry: &mut AssetRegistry, _window_size: &Vec2) -> Result<(), String> {
        Err(format!("Can not set text of a ui rectangle. Tried with [{}]", text))
    }
}

impl Rectangle {
    pub fn new(builder: RectangleBuilder, asset_registry: &mut AssetRegistry, window_size: &Vec2) -> Result<Self, String> {
        let shader_builder = match builder.shader_builder {
            Some(custom_shader_builder) => custom_shader_builder,
            None => Self::default_shader_builder(),
        };

        let shader_id = asset_registry.load_shader(shader_builder)?;
        let material_id = asset_registry.load_material(&shader_id)?;

        let vertices: [Vertex; 4] = [
            Vertex([-builder.width / 2.0, -builder.height / 2.0]), // bottom left
            Vertex([builder.width / 2.0, -builder.height / 2.0]), // bottom right
            Vertex([builder.width / 2.0, builder.height / 2.0]), // top right
            Vertex([-builder.width / 2.0, builder.height / 2.0])  // top left
        ];

        let vao = Vao::new();
        vao.bind();
        
        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&vertices, gl::STATIC_DRAW);

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&RECTANGLE_INDICES, gl::STATIC_DRAW);

        let position_attribute = asset_registry.get_shader_by_id(&shader_id).unwrap().get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, Vertex::0);

        let world_data = WorldElementData::new(
            builder.position,
            builder.z_index, 
            Vec2::new(builder.width, builder.height) ,
            window_size
        );

        Ok(Self { 
            vao, 
            _vbo: vbo,
            ebo,
            material_id,
            color: builder.color,
            world_data,
        })
    }

    pub fn default_shader_builder() -> ShaderBuilder {
        ShaderBuilder::new()
            .with_vertex_shader_path("./assets/shaders/ui/rectangle.vert".to_string())
            .with_fragment_shader_path("./assets/shaders/ui/rectangle.frag".to_string())
    }
}

pub struct RectangleBuilder {
    color: (u8, u8, u8),
    position: Position,
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
            position: Position::FixedCenter,
            width: 100.0,
            height: 40.0,
            z_index: 10.0,
        }
    }

    pub fn with_color(mut self, color: (u8, u8, u8)) -> Self {
        self.color = color;
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
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
            log::engine_warn(format!("did not set RectangleBuilder z_index {} because it's not a valid z-index", z_index));
        }

        self
    }
}

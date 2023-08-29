use glam::Vec2;

use crate::{graphics::{renderer::buffer::{Buffer, Vao}, shader::ShaderBuilder, ui::{interface::{is_valid_z_index, map_z_index_for_shader}, element::{world_element_data::WorldElementData, ui_element::UiElement, AnchorPoint, AnchorElementData}, Position, ElementRegistry, bounds_2d::Bounds2d}, material::Material, Color, texture::{vertex_coordinates, Texture}}, set_attribute, error::opengl, asset_manager::AssetManager, log, ResourceId};
use crate::graphics::shapes::RECTANGLE_INDICES;

use super::rectangle_border::{Border, BorderSize};

type VertexPosition = [f32; 2];
type TextureCoordinates = [f32; 2];
pub struct Vertex(VertexPosition, TextureCoordinates);

pub struct Rectangle {
    vao: Vao,
    _vbo: Buffer,
    ebo: Buffer,

    material_id: ResourceId<Material>,
    world_data: WorldElementData,

    color: Color,
    border: Border,
}

impl UiElement for Rectangle {
    fn draw(&self, asset_manager: &mut AssetManager, window_size: &Vec2, pixel_density: f32) {
        if !self.world_data.show {
            return
        }

        let shader_id = asset_manager.get_material_by_id(&self.material_id).unwrap().shader_id.duplicate();
        let shader = asset_manager.get_shader_by_id(&shader_id).unwrap();

        shader.apply();
        self.vao.bind();

        let fragment_shader_size = self.world_data.size() * self.world_data.scale();
        let shader_position = self.world_data.shader_position();

        let border_bounds = Bounds2d::some(
            fragment_shader_size.y / 2.0 + shader_position.y - self.border.size.top * self.world_data.scale().y,
            fragment_shader_size.x / 2.0 + shader_position.x - self.border.size.right * self.world_data.scale().y,
            -fragment_shader_size.y / 2.0 + shader_position.y + self.border.size.bottom * self.world_data.scale().y,
            -fragment_shader_size.x / 2.0 + shader_position.x + self.border.size.left * self.world_data.scale().y,
        );

        shader.set_uniform("color", self.color.to_normalised_rgba_tuple());
        shader.set_uniform("scale", (self.world_data.scale().x, self.world_data.scale().y));
        shader.set_uniform("zIndex", map_z_index_for_shader(self.world_data.z_index));
        shader.set_uniform("worldPosition", shader_position);
        shader.set_uniform("drawBounds", self.world_data.draw_bounds.for_fragment_shader(window_size, pixel_density));

        shader.set_uniform("borderColor", self.border.color.to_normalised_rgba_tuple());
        shader.set_uniform("borderBounds", border_bounds.for_fragment_shader(&window_size, pixel_density));

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.ebo.data_size as i32, gl::UNSIGNED_INT, core::ptr::null());
        }

        opengl::gl_check_errors();
    }

    fn material_id(&self) -> &ResourceId<Material> {
        &self.material_id
    }

    fn type_name(&self) -> &str {
        "rectangle"
    }

    fn world_data(&self) -> &WorldElementData { &self.world_data }
    fn mut_world_data(&mut self) -> &mut WorldElementData { &mut self.world_data }

    fn handle_window_resize(&mut self, new_window_size: &Vec2) {
        self.world_data.handle_window_resize(new_window_size);
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Rectangle {
    pub fn default_shader_builder() -> ShaderBuilder {
        ShaderBuilder::new()
            .with_vertex_shader_path("./assets/shaders/ui/rectangle.vert".to_string())
            .with_fragment_shader_path("./assets/shaders/ui/rectangle.frag".to_string())
    }

    pub fn default_textured_shader_builder() -> ShaderBuilder {
        ShaderBuilder::new()
            .with_vertex_shader_path("./assets/shaders/ui/rectangle-textured.vert".to_string())
            .with_fragment_shader_path("./assets/shaders/ui/rectangle-textured.frag".to_string())
    }

    pub fn set_width(&mut self, width: f32, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        let height = self.world_data.height();
        self.set_size(Vec2::new(width, height), window_size, anchor_element_data);
    }

    pub fn set_height(&mut self, height: f32, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        let width = self.world_data.width();
        self.set_size(Vec2::new(width, height), window_size, anchor_element_data);
    }

    pub fn set_size(&mut self, size: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        self.vao.bind();
        self._vbo.update_data(&Self::create_vertices(&size));

        self.world_data.set_size(size, window_size, anchor_element_data);
    }

    pub fn set_border_size(&mut self, border_size: f32) {
        self.border.size.set_universal(border_size);
    }

    pub fn set_border_sizes(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
        self.border.size.set_individual(top, right, bottom, left);
    }

    pub fn set_border_color(&mut self, border_color: Color) {
        self.border.color = border_color;
    }

    fn create_vertices(size: &Vec2) -> [Vertex; 4] {
        [
            Vertex([-size.x / 2.0,  -size.y / 2.0], vertex_coordinates::FULL_BOTTOM_LEFT), // bottom left
            Vertex([size.x / 2.0,   -size.y / 2.0], vertex_coordinates::FULL_BOTTOM_RIGHT), // bottom right
            Vertex([size.x / 2.0,   size.y / 2.0],  vertex_coordinates::FULL_TOP_RIGHT), // top right
            Vertex([-size.x / 2.0,  size.y / 2.0],  vertex_coordinates::FULL_TOP_LEFT),  // top left
        ]
    }
}

pub enum RectangleTexture {
    Id(ResourceId<Texture>),
    Path(String),
}

impl RectangleTexture {
    fn upload(&self, material_id: &ResourceId<Material>, asset_manager: &mut AssetManager) -> Result<ResourceId<Texture>, String> {
        match self {
            RectangleTexture::Id(texture_id) => {
                asset_manager.add_material_texture(&material_id, &texture_id);
                Ok(texture_id.duplicate())
            },
            RectangleTexture::Path(texture_path) => {
                match asset_manager.load_texture(texture_path) {
                    Ok(texture_id) => {
                        asset_manager.add_material_texture(&material_id, &texture_id);
                        Ok(texture_id)
                    },
                    Err(err) => {
                        Err(err)
                    },
                }
            },
        }
    }
}

pub struct RectangleBuilder {
    color: Color,
    position: Position,
    shader_builder: Option<ShaderBuilder>,
    size: Vec2,
    z_index: f32,
    scale: Vec2,
    hidden: bool,
    texture: Option<RectangleTexture>,
    border: Border,
}

impl RectangleBuilder {
    pub fn new() -> Self {
        Self {
            color: Color::Rgb(126, 126, 126), // gray
            shader_builder: None,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            size: Vec2::new(100.0, 40.0),
            z_index: 10.0,
            scale: Vec2::ONE,
            hidden: false,
            texture: None,
            border: Border {
                color: Color::rgb_black(),
                size: BorderSize::zero(),
            },
        }
    }

    pub fn build(&self, asset_manager: &mut AssetManager, element_registry: &ElementRegistry) -> Result<Rectangle, String> {
        let shader_builder = match self.shader_builder.clone() {
            Some(custom_shader_self) => custom_shader_self,
            None => self.default_shader_builder(),
        };

        let shader_id = asset_manager.load_shader(shader_builder)?;
        let material_id = asset_manager.load_material(&shader_id)?;

        if let Some(texture_mode) = &self.texture {
            _ = texture_mode.upload(&material_id, asset_manager).map_err(|err| {
                log::engine_err(format!("failed to add rectangle texture: {}", err));
            });
        }

        let vao = Vao::new();
        vao.bind();
        
        let mut vbo = Buffer::new_vbo();
        vbo.set_data(&Rectangle::create_vertices(&self.size), gl::STATIC_DRAW);

        let mut ebo = Buffer::new_ebo();
        ebo.set_data(&RECTANGLE_INDICES, gl::STATIC_DRAW);

        let position_attribute = asset_manager.get_shader_by_id(&shader_id).unwrap().get_attribute_location("position")
            .expect("Could not get position attribute");
        set_attribute!(vao, position_attribute, Vertex::0);

        if self.texture.is_some() {
            let texture_coordinates_attribute = asset_manager.get_shader_by_id(&shader_id).unwrap().get_attribute_location("textureCoordinates")
                .expect("Could not get position attribute");
            set_attribute!(vao, texture_coordinates_attribute, Vertex::1);
        }

        let mut world_data = WorldElementData::new(
            self.position,
            self.z_index, 
            self.size,
            self.scale,
            element_registry
        );
        world_data.show = !self.hidden;

        Ok(Rectangle { 
            vao, 
            _vbo: vbo,
            ebo,
            material_id,
            color: self.color.clone(),
            world_data,
            border: self.border.clone(),
        })
    }

    fn default_shader_builder(&self) -> ShaderBuilder {
        match self.texture {
            Some(_) => Rectangle::default_textured_shader_builder(),
            None => Rectangle::default_shader_builder(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn without_color(mut self) -> Self {
        self.color = Color::transparent();
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
        self.size.x = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.size.y = height;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
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

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn with_texture(mut self, texture: RectangleTexture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_border_size(mut self, border_size: f32) -> Self {
        self.border.size.set_universal(border_size);
        self
    }

    pub fn with_border_sizes(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.border.size.set_individual(top, right, bottom, left);
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border.color = border_color;
        self
    }
}

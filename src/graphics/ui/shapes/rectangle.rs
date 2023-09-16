use std::collections::HashMap;

use glam::{Vec2, Vec4};

use crate::{graphics::{renderer::buffer::{Buffer, Vao}, shader::ShaderBuilder, ui::{interface::{is_valid_z_index, map_z_index_for_shader}, element::{world_element_data::WorldElementData, ui_element::UiElement, AnchorPoint, AnchorElementData}, Position, ElementRegistry, bounds_2d::Bounds2d, UiTexture}, material::Material, Color}, set_attribute, error::opengl, asset_manager::AssetManager, log, ResourceId};
use crate::graphics::shapes::RECTANGLE_INDICES;

use super::rectangle_border::{Border, BorderSize, BorderRadius};

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
    texture_padding: f32,

    custom_shader_uniforms_vec4: HashMap<String, Vec4>,
}

impl UiElement for Rectangle {
    fn draw(&self, asset_manager: &mut AssetManager, window_size: &Vec2, pixel_density: f32) {
        if !self.world_data.show {
            return
        }

        self.activate(asset_manager);

        let shader = asset_manager.get_material_shader(&self.material_id).unwrap();
        
        let fragment_shader_size = self.world_data.size() * self.world_data.scale();
        let shader_position = self.world_data.shader_position();

        let border_bounds = Bounds2d::some(
            fragment_shader_size.y / 2.0 + shader_position.y - self.border.size.top * self.world_data.scale().y,
            fragment_shader_size.x / 2.0 + shader_position.x - self.border.size.right * self.world_data.scale().y,
            -fragment_shader_size.y / 2.0 + shader_position.y + self.border.size.bottom * self.world_data.scale().y,
            -fragment_shader_size.x / 2.0 + shader_position.x + self.border.size.left * self.world_data.scale().y,
        );

        let element_bounds = Bounds2d::some(
            fragment_shader_size.y / 2.0 + shader_position.y,
            fragment_shader_size.x / 2.0 + shader_position.x,
            -fragment_shader_size.y / 2.0 + shader_position.y,
            -fragment_shader_size.x / 2.0 + shader_position.x,
        );

        shader.set_uniform("color", self.color.to_normalised_rgba_tuple());
        shader.set_uniform("scale", (self.world_data.scale().x, self.world_data.scale().y));
        shader.set_uniform("zIndex", map_z_index_for_shader(self.world_data.z_index));
        shader.set_uniform("worldPosition", shader_position);
        shader.set_uniform("drawBounds", self.world_data.draw_bounds.for_fragment_shader(window_size, pixel_density));

        shader.set_uniform("elementBounds", element_bounds.for_fragment_shader(window_size, pixel_density));
        shader.set_uniform("borderRadius", self.border.radius.to_vec() * pixel_density);

        shader.set_uniform("borderColor", self.border.color.to_normalised_rgba_tuple());
        shader.set_uniform("borderBounds", border_bounds.for_fragment_shader(&window_size, pixel_density));
        shader.set_uniform("borderSize", self.border.size.vec4() * pixel_density);

        for (name, value) in self.custom_shader_uniforms_vec4.iter() {
            shader.set_uniform(name, *value);
        }

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
        ShaderBuilder::new(
            "./assets/shaders/ui/rectangle.vert", 
            "./assets/shaders/ui/rectangle.frag"
        )
    }

    pub fn default_textured_shader_builder() -> ShaderBuilder {
        ShaderBuilder::new(
            "./assets/shaders/ui/rectangle-textured.vert", 
            "./assets/shaders/ui/rectangle-textured.frag"
        )
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
        self._vbo.update_data(&Self::create_vertices(&size, self.texture_padding));

        self.world_data.set_size(size, window_size, anchor_element_data);
    }

    pub fn get_border(&self) -> &Border {
        &self.border
    }
    pub fn get_mut_border(&mut self) -> &mut Border {
        &mut self.border
    }

    pub fn set_texture_padding(&mut self, texture_padding: f32) {
        self.texture_padding = texture_padding;

        self.vao.bind();
        self._vbo.update_data(&Self::create_vertices(&self.world_data.size(), self.texture_padding));
    }

    // TODO padding per side
    // TODO higher padding than the size makes the texture it's x/y inverted. Limit this.
    fn create_vertices(size: &Vec2, texture_padding: f32) -> [Vertex; 4] {
        // A texture over the full size of the rectangle has a range of 1 (from 0.0 to 1.0).
        // If this range goes up, the texture gets smaller. To calculate the size of the image,
        // we can use 100% / range. For example, if the range is 2, the texture its size is 50%.
        let range_x = 1.0 / (1.0 - (texture_padding * 2.0 / size.x));
        let range_y = 1.0 / (1.0 - (texture_padding * 2.0 / size.y));

        let range_x_min = (-range_x + 1.0) / 2.0;
        let range_x_max = 1.0 + (range_x - 1.0) / 2.0;
        let range_y_min = (-range_y + 1.0) / 2.0;
        let range_y_max = 1.0 + (range_y - 1.0) / 2.0;

        let texture_coords_bottom_left = [range_x_min, range_y_max];
        let texture_coords_bottom_right = [range_x_max, range_y_max];
        let texture_coords_top_right = [range_x_max, range_y_min];
        let texture_coords_top_left = [range_x_min, range_y_min];

        [
            Vertex([-size.x / 2.0,  -size.y / 2.0], texture_coords_bottom_left), // bottom left
            Vertex([size.x / 2.0,   -size.y / 2.0], texture_coords_bottom_right), // bottom right
            Vertex([size.x / 2.0,   size.y / 2.0],  texture_coords_top_right), // top right
            Vertex([-size.x / 2.0,  size.y / 2.0],  texture_coords_top_left),  // top left
        ]
    }

    /// bind and activate resources
    fn activate(&self, asset_manager: &mut AssetManager) {
        asset_manager.activate_material(&self.material_id);
        self.vao.bind();
    }

    fn mut_custom_shader_uniforms_vec4(&mut self) -> &HashMap<String, Vec4> {
        &mut self.custom_shader_uniforms_vec4
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
    texture: Option<UiTexture>,
    border: Border,
    texture_padding: f32,
    custom_shader_uniforms_vec4: HashMap<String, Vec4>,
}

impl RectangleBuilder {
    pub fn new() -> Self {
        Self {
            color: Color::gray(),
            shader_builder: None,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            size: Vec2::new(100.0, 40.0),
            z_index: 10.0,
            scale: Vec2::ONE,
            hidden: false,
            texture: None,
            border: Border {
                color: Color::black(),
                size: BorderSize::zero(),
                radius: BorderRadius::zero(),
            },
            texture_padding: 0.0,
            custom_shader_uniforms_vec4: Default::default(),
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
        vbo.set_data(&Rectangle::create_vertices(&self.size, self.texture_padding), gl::STATIC_DRAW);

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
            texture_padding: self.texture_padding.clone(),
            custom_shader_uniforms_vec4: self.custom_shader_uniforms_vec4.clone(),
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

    pub fn with_texture(mut self, texture: UiTexture) -> Self {
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

    pub fn with_border_radius(mut self, border_radius: f32) -> Self {
        self.border.radius.set_universal(border_radius);
        self
    }

    pub fn with_border_radiuses(mut self, top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        self.border.radius.set_individual(top_left, top_right, bottom_right, bottom_left);
        self
    }

    pub fn with_texture_padding(mut self, texture_padding: f32) -> Self {
        self.texture_padding = texture_padding;
        self
    }

    pub fn with_custom_shader_uniform_vec4(mut self, name: impl Into<String>, value: Vec4) -> Self {
        self.custom_shader_uniforms_vec4.insert(name.into(), value);
        self
    }
}

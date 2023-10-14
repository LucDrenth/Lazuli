use glam::Vec2;

use crate::{graphics::{ui::{ElementRegistry, UiElementId, shapes::{RectangleBuilder, Rectangle}, UiTexture, Position, interface::{is_valid_z_index, self, WidgetRegistry}, widget::UiWidget, bounds_2d::Bounds2d, UiUpdateTargets, UpdateTargetCollection}, Color, shader::ShaderBuilder, texture::Texture}, asset_manager::AssetManager, ResourceId, log};

pub struct Icon {
    rectangle_element_id: ResourceId<UiElementId>,
    color: Option<Color>,
    z_index: f32,
}

impl UiWidget for Icon {
    fn get_all_element_ids(&self, _widget_registry: &WidgetRegistry) -> Vec<ResourceId<UiElementId>> {
        vec![self.rectangle_element_id]
    }

    fn get_main_element_id(&self, _widget_registry: &WidgetRegistry) -> ResourceId<UiElementId> {
        self.rectangle_element_id
    }

    fn z_index(&self) -> f32 {
        self.z_index
    }
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.z_index = z_index;
        _ = element_registry.set_element_z_index(&self.rectangle_element_id, z_index);

        UiUpdateTargets::default()
    }

    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        _ = element_registry.set_element_position(&self.rectangle_element_id, position);
        UpdateTargetCollection::default()
    }

    fn set_draw_bounds(&self, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d> {
        _ = element_registry.set_element_draw_bounds(&self.rectangle_element_id, draw_bounds);
        UiUpdateTargets::default()
    }

    fn set_width(&self, width: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        _ = element_registry.set_rectangle_width(&self.rectangle_element_id, width);
        UiUpdateTargets::default()
    }
    fn set_height(&self, height: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        _ = element_registry.set_rectangle_height(&self.rectangle_element_id, height);
        UiUpdateTargets::default()
    }
    fn set_size(&self, size: Vec2, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2> {
        _ = element_registry.set_rectangle_size(&self.rectangle_element_id, size);
        UiUpdateTargets::default()
    }

    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        _ = element_registry.set_element_visibility(&self.rectangle_element_id, visible);
        UiUpdateTargets::default()
    }
}

enum IconSize {
    /// Scale height based on the given width
    Width(f32),
    Height(f32),
    Fixed(Vec2),
}
impl IconSize {
    fn to_fixed(&self, texture: &Texture) -> Vec2 {
        match self {
            IconSize::Width(width) => Vec2::new(
                *width, 
                texture.height() / texture.width() * width,
            ),
            IconSize::Height(height) => Vec2::new(
                texture.width() / texture.height() * height,
                *height
            ),
            IconSize::Fixed(size) => size.clone(),
        }
    }
}

pub struct IconBuilder {
    texture: UiTexture,
    color: Option<Color>,
    background_color: Color,
    size: IconSize,
    z_index: f32,
    position: Position,
    shader_builder: Option<ShaderBuilder>,
    padding: f32,
    is_visible: bool,
}

impl IconBuilder {
    pub fn new() -> Self {
        Self { 
            texture: UiTexture::path("./assets/images/icons/arrow-down.png"),
            color: Some(interface::default_text_color()),
            background_color: Color::transparent(),
            size: IconSize::Height(interface::default_font_size()),
            z_index: 10.0,
            position: Default::default(),
            shader_builder: None,
            padding: 0.0,
            is_visible: true,
        }
    }

    pub fn build(&self, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<Icon, String> {
        // We need to upload the texture before building the rectangle because we need to know the texture size so we can
        // auto scale the size
        let shader_builder = match self.shader_builder.clone() {
            Some(custom_shader_self) => custom_shader_self,
            None => Rectangle::default_textured_shader_builder(),
        };
        let shader_id = asset_manager.load_shader(shader_builder)?;
        let material_id = asset_manager.load_material(&shader_id)?;
        let texture_id = self.texture.upload(&material_id, asset_manager)?;

        let size = self.size.to_fixed(asset_manager.get_texture_by_id(&texture_id).unwrap());

        let mut rectangle_builder = RectangleBuilder::new()
            .with_color(self.background_color.clone())
            .with_texture(UiTexture::Id(texture_id))
            .with_texture_padding(self.padding)
            .with_size(size)
            .with_z_index(self.z_index)
            .with_position(self.position)
            .with_custom_shader_value_f32("rotation", 0.)
            .with_visibility(self.is_visible)
        ;

        match &self.color {
            Some(color) => {
                rectangle_builder = rectangle_builder
                    .with_shader_builder(ShaderBuilder::new("./assets/shaders/ui/rectangle-icon.vert", "./assets/shaders/ui/rectangle-icon.frag"))
                    .with_custom_shader_value_vec4("iconColor", color.to_normalised_rgba_vec4())
            },
            None => (),
        }

        let rectangle_element_id = element_registry.create_rectangle(&rectangle_builder, asset_manager)?;

        let icon = Icon{
            rectangle_element_id,
            color: self.color.clone(),
            z_index: self.z_index,
        };

        Ok(icon)
    }

    pub fn with_texture(mut self, texture: UiTexture) -> Self {
        self.texture = texture;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    // Use the colors of the icon texture
    pub fn with_original_texture_color(mut self) -> Self {
        self.color = None;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    /// Set auto scaling height
    pub fn with_width(mut self, width: f32) -> Self {
        self.size = IconSize::Width(width);
        self
    }
    /// Set auto scaling width
    pub fn with_height(mut self, height: f32) -> Self {
        self.size = IconSize::Height(height);
        self
    }
    /// Set a fixed icon size
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = IconSize::Fixed(size);
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        // Add 1 because the given z_index is for the lowest element, and higher elements may go up to 1 higher
        if is_valid_z_index(z_index + 1.0) {
            self.z_index = z_index;
        } else {
            log::engine_warn(format!("Did not set ButtonBuilder z_index {} because it's not a valid z-index", z_index));
        }

        self
    }

    /// Use a custom shader
    pub fn with_shader_builder(mut self, shader_builder: ShaderBuilder) -> Self {
        self.shader_builder = Some(shader_builder);
        self
    }
    /// Default to custom shader
    pub fn without_shader_builder(mut self) -> Self {
        self.shader_builder = None;
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.is_visible = visible;
        self
    }
}

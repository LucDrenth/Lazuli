use glam::Vec2;

use crate::{graphics::{ui::{TextBuilder, ElementRegistry, self, interface::{is_valid_z_index, self, WidgetRegistry}, Position, element::{AnchorPoint, ui_element::UiElement}, widget::UiWidget, UiElementId, text::TextAlign, Padding, bounds_2d::Bounds2d, UiUpdateTargets}, font::PlainBitmapBuilder, Color}, asset_manager::AssetManager, input::{Input, InputAction, MouseButton}, log, ResourceId};

pub struct Button {
    text_element_id: ResourceId<UiElementId>,
    background_element_id: ResourceId<UiElementId>,
    mouse_action_to_activate: InputAction,
    mouse_button_to_activate: MouseButton,
    z_index: f32,
    width: f32,
    height: f32,
    text_align: TextAlign,
    padding: Padding,
}

impl UiWidget for Button {
    fn get_all_element_ids(&self, _widget_registry: &WidgetRegistry) -> Vec<ResourceId<UiElementId>> {
        vec![
            self.background_element_id,
            self.text_element_id,
        ]
    }

    /// Background is the main element. It defines the position and size of the slider
    fn get_main_element_id(&self, _widget_registry: &WidgetRegistry) -> ResourceId<UiElementId> {
        self.background_element_id.clone()
    }

    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Position> {
        _ = element_registry.set_element_position(&self.background_element_id, position);
        _ = element_registry.set_element_position(&self.text_element_id, Position::ElementAnchor(
            text_align_to_anchor_point(&self.text_align, &self.padding), 
            self.background_element_id)
        );

        UiUpdateTargets::default()
    }

    fn z_index(&self) -> f32 {
        self.z_index
    }
    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.z_index = z_index;
        _ = element_registry.set_element_z_index(&self.background_element_id, z_index);
        _ = element_registry.set_element_z_index(&self.text_element_id, z_index + 0.01);

        UiUpdateTargets::default()
    }

    fn set_draw_bounds(&self, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d> {
        _ = element_registry.set_element_draw_bounds(&self.background_element_id, draw_bounds);
        _ = element_registry.set_element_draw_bounds(&self.text_element_id, draw_bounds);
        UiUpdateTargets::default()
    }

    fn set_width(&self, width: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        _ = element_registry.set_rectangle_width(&self.background_element_id, width);
        UiUpdateTargets::default()
    }
    fn set_height(&self, height: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        _ = element_registry.set_rectangle_height(&self.background_element_id, height);
        UiUpdateTargets::default()
    }
    fn set_size(&self, size: Vec2, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2> {
        _ = element_registry.set_rectangle_size(&self.background_element_id, size);
        UiUpdateTargets::default()
    }

    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        _ = element_registry.set_element_visibility(&self.background_element_id, visible);
        _ = element_registry.set_element_visibility(&self.text_element_id, visible);

        UiUpdateTargets::default()
    }
}

impl Button {
    pub fn is_hovered(&self, input: &Input, element_registry: &ElementRegistry) -> bool {
        element_registry.is_element_hovered(&self.background_element_id, input)
    }

    pub fn is_clicked(&self, input: &Input, element_registry: &ElementRegistry) -> bool {
        !element_registry.is_any_element_dragged() && element_registry.is_element_clicked(
            &self.background_element_id, 
            self.mouse_button_to_activate,
            &self.mouse_action_to_activate, 
            input
        )
    }

    pub fn set_scale(&mut self, scale: Vec2, element_registry: &mut ElementRegistry) -> Result<(), String> {
        element_registry.set_element_scale(&self.background_element_id, scale)?;
        element_registry.set_element_scale(&self.text_element_id, scale)?;
        Ok(())
    }

    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.height }
    pub fn text_element_id(&self) -> ResourceId<UiElementId> { self.text_element_id.clone() }
    pub fn padding(&self) -> &Padding { &self.padding }

    pub fn set_background_color(&self, color: Color, element_registry: &mut ElementRegistry) -> Result<(), String> {
        element_registry.set_element_color(&self.background_element_id, color)
    }

    pub fn set_text_color(&self, color: Color, element_registry: &mut ElementRegistry) -> Result<(), String> {
        element_registry.set_element_color(&self.text_element_id, color)
    }
}

pub struct ButtonBuilder {
    background_color: Color,
    text_color: Color,
    font_path: Option<String>,
    padding: Padding,
    z_index: f32,
    position: Position,
    font_size: f32,
    scale: Vec2,
    mouse_action_to_activate: InputAction,
    mouse_button_to_activate: MouseButton,
    width: Option<f32>,
    height: Option<f32>,
    is_visible: bool,
    text_align: TextAlign,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            background_color: interface::default_element_background_color(),
            text_color: interface::default_text_color(),
            font_path: None,
            padding: Padding::HorizontalVertical(15.0, 8.0),
            z_index: 10.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            font_size: interface::default_font_size(),
            scale: Vec2::ONE,
            mouse_action_to_activate: InputAction::Down,
            mouse_button_to_activate: MouseButton::Left,
            width: None,
            height: None,
            is_visible: true,
            text_align: TextAlign::Center,
        }
    }

    pub fn build(&self, label: impl Into<String>, element_registry: &mut ElementRegistry, asset_manager: &mut AssetManager) -> Result<Button, String> {
        let font_id = match &self.font_path {
            Some(font_path) => asset_manager.load_font(PlainBitmapBuilder::new()
                .with_font_file_path(font_path.clone())
                .with_font_size(50.0)
                , None)?,
            None => interface::default_font(asset_manager)?,
        };

        let mut text = TextBuilder::new()
            .with_color(self.text_color.clone())
            .with_z_index(self.z_index + 0.01)
            .with_font_size(self.font_size)
            .with_scale(self.scale)
            .with_visibility(self.is_visible)
            .build(label, &font_id, asset_manager, element_registry)?;
        
        let button_width = match self.width {
            Some(width) => width,
            None => text.world_data().width() + self.padding.horizontal(),
        };
        let button_height = match self.height {
            Some(height) => height,
            None => text.world_data().height() + self.padding.vertical(),
        };

        let background = ui::shapes::RectangleBuilder::new()
            .with_width(button_width)
            .with_height(button_height)
            .with_color(self.background_color.clone())
            .with_z_index(self.z_index)
            .with_position(self.position)
            .with_scale(self.scale)
            .with_visibility(self.is_visible)
            .build(asset_manager, element_registry)?;
        let background_element_id = element_registry.add_rectangle(background);

        let window_size = element_registry.size().clone();
        let anchor_element_data = element_registry.get_anchor_data(&background_element_id)?;
        text.mut_world_data().set_position(
            Position::ElementAnchor(
                text_align_to_anchor_point(&self.text_align, &self.padding), 
                background_element_id
            ), 
            window_size, 
            Some(anchor_element_data)
        );
        let text_element_id = element_registry.add_text(text);

        Ok(Button {
            text_element_id,
            background_element_id,
            mouse_action_to_activate: self.mouse_action_to_activate,
            mouse_button_to_activate: self.mouse_button_to_activate,
            z_index: self.z_index,
            width: button_width,
            height: button_height,
            text_align: self.text_align,
            padding: self.padding.clone(),
        })
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn with_text_color(mut self, text_color: Color) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn with_font_path(mut self, font_path: String) -> Self {
        self.font_path = Some(font_path);
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
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

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_mouse_action_to_activate(mut self, mouse_action_to_activate: InputAction) -> Self {
        self.mouse_action_to_activate = mouse_action_to_activate;
        self
    }

    pub fn with_mouse_button_to_activate(mut self, mouse_button_to_activate: MouseButton) -> Self {
        self.mouse_button_to_activate = mouse_button_to_activate;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.is_visible = visible;
        self
    }

    pub fn with_text_align(mut self, text_align: TextAlign) -> Self {
        self.text_align = text_align;
        self
    }
}

fn text_align_to_anchor_point(text_align: &TextAlign, padding: &Padding) -> AnchorPoint {
    match text_align {
        TextAlign::Left => AnchorPoint::LeftInside(padding.left()),
        TextAlign::Center => AnchorPoint::Center,
        TextAlign::Right => AnchorPoint::RightInside(padding.right()),
    }
}

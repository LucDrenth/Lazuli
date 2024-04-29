use glam::Vec2;

use crate::graphics::ui::{ElementRegistry, bounds_2d::Bounds2d};

use super::{input_events::InputEventHandlers, AnchorElementData, Position};

/// World space data about size and positioning of a UI element
#[derive(Clone, Copy)]
pub struct WorldElementData {
    size: Vec2, // given in world space (pixels)
    position: Vec2, // the world space coordinates at which we render the center of the element. (0,0) is the center of the screen
    position_type: Position,
    pub position_transform: Vec2, // Where self.position is meant to be static, this position transform can be used to move the element around
    pub z_index: f32,
    scale: Vec2,
    pub show: bool,
    pub draw_bounds: Bounds2d,
    pub event_handlers: InputEventHandlers,
}

impl WorldElementData {
    pub fn new(position_type: Position, z_index: f32, size: Vec2, scale: Vec2, element_registry: &ElementRegistry) -> Self {
        let mut result = Self {
            size,
            position: Vec2::ZERO, // will be calculated later in this function with calculate_position
            position_type,
            position_transform: Vec2::ZERO,
            z_index,
            scale,
            show: true,
            draw_bounds: Bounds2d::none(),
            event_handlers: InputEventHandlers::new(),
        };

        let anchor_element_data = position_type.get_anchor_element_data(element_registry);
        result.calculate_position(element_registry.size().clone(), anchor_element_data);

        result
    }

    /// Useful for tests
    pub fn new_mock() -> Self {
        Self {
            size: Vec2::ZERO,
            position: Vec2::ZERO,
            position_type: Position::Fixed(0., 0.),
            position_transform: Vec2::ZERO,
            z_index: 0.,
            scale: Vec2::ZERO,
            show: true,
            draw_bounds: Bounds2d::none(),
            event_handlers: InputEventHandlers::new(),
        }
    }

    pub fn shader_position(&self) -> Vec2 {
        self.position + self.position_transform
    }
    pub fn calculate_position(&mut self, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        self.position = self.position_type.to_coordinates(self.size * self.scale, window_size, anchor_element_data);
    }
    pub fn set_position(&mut self, position: Position, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        self.position_type = position;

        self.calculate_position(window_size, anchor_element_data);
    }

    pub fn set_size(&mut self, size: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        self.size = size;
        self.calculate_position(window_size, anchor_element_data);
    }
    pub fn size(&self) -> Vec2 { self.size }
    pub fn width(&self) -> f32 { self.size.x }
    pub fn height(&self) -> f32 { self.size.y }

    pub fn set_scale(&mut self, scale: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        self.scale = scale;
        self.calculate_position(window_size, anchor_element_data);
    }
    pub fn scale(&self) -> Vec2 { self.scale }

    // Check if the given position is within this world element
    pub fn is_within(&self, position: Vec2) -> bool {
        self.draw_bounds.is_within(position) && self.is_within_current_position(position)
    }

    fn is_within_current_position(&self, position: Vec2) -> bool {
        let size = self.size * self.scale;

        let current_position = self.position + self.position_transform;

        return position.x >= current_position.x - size.x / 2.0
            && position.x < current_position.x + size.x / 2.0
            && position.y >= current_position.y - size.y / 2.0
            && position.y < current_position.y + size.y / 2.0
    }

    pub fn handle_window_resize(&mut self, _new_window_size: &Vec2) {
        // If we implement screen anchor points we might want to use this function
    }

    /// Screen position
    pub fn position(&self) -> Vec2 { self.position + self.position_transform }
    pub fn position_type(&self) -> &Position { &self.position_type }
}

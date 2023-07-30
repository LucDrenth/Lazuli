use glam::Vec2;

use crate::graphics::ui::Interface;

use super::{Position, AnchorElementData};

/// World space data about size and positioning of a UI element
#[derive(Clone, Copy)]
pub struct WorldElementData {
    size: Vec2, // given in world space (pixels)
    position: Vec2, // the world space coordinates at which we render the center of the element. (0,0) is the center of the screen
    position_type: Position,
    z_index: f32,
    pub scale: Vec2,
}

impl WorldElementData {
    pub fn new(position_type: Position, z_index: f32, size: Vec2, scale: Vec2, interface: &Interface) -> Self {
        let mut result = Self {
            size,
            position: Vec2::ZERO, // to be calculated with calculate_position
            position_type,
            z_index,
            scale,
        };

        let mut anchor_element_data = None;

        match position_type {
            Position::Fixed(_, _) => (),
            Position::ScreenAnchor(_) => (),
            Position::ElementAnchor(_, element_id) => {
                anchor_element_data = Some(interface.get_anchor_data(element_id).unwrap());
            },
        }

        result.calculate_position(interface.size().clone(), anchor_element_data);

        result
    }

    pub fn shader_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn calculate_position(&mut self, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        self.position = self.position_type.to_coordinates(self.size * self.scale, window_size, anchor_element_data);
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    pub fn set_scale(&mut self, scale: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) {
        self.scale = scale;
        self.calculate_position(window_size, anchor_element_data);
    }

    // Check if the given position is within this world element
    pub fn is_within(&self, position: Vec2) -> bool {
        let size = self.size * self.scale;

        return position.x >= self.position.x - size.x / 2.0
            && position.x < self.position.x + size.x / 2.0
            && position.y >= self.position.y - size.y / 2.0
            && position.y < self.position.y + size.y / 2.0
    }

    pub fn handle_window_resize(&mut self, _new_window_size: &Vec2) {
        // If we implement screen anchor points we might want to use this function
    }

    // Put our element at the center of the given element (element_to_center_on)
    pub fn center_at(&mut self, element_to_center_on: &Self, _window_size: &Vec2) {
        self.position = element_to_center_on.position.clone();
    }

    pub fn position(&self) -> &Vec2 { &self.position }
    pub fn position_type(&self) -> &Position { &self.position_type }
    pub fn size(&self) -> &Vec2 { &self.size }
    pub fn width(&self) -> f32 { self.size.x }
    pub fn height(&self) -> f32 { self.size.y }
    pub fn z_index(&self) -> f32 { self.z_index }
}

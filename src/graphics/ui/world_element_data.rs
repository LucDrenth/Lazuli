use glam::Vec2;

use super::Position;

/// World space data about size and positioning of a UI element
pub struct WorldElementData {
    size: Vec2, // given in world space (pixels)
    final_coordinates: Vec2, // the world space coordinates at which we render the center of the element. (0,0) is the center of the screen
    position_type: Position,
    z_index: f32,
}

impl WorldElementData {
    pub fn new(position_type: Position, z_index: f32, size: Vec2, window_size: &Vec2) -> Self {
        let final_coordinates = position_type.shader_coordinates(&size, window_size);

        Self {
            size,
            final_coordinates,
            position_type,
            z_index,
        }
    }

    pub fn final_coordinates(&self) -> &Vec2 {
        &self.final_coordinates
    }

    pub fn shader_coordinates(&self) -> (f32, f32) {
        (self.final_coordinates.x, self.final_coordinates.y)
    }

    // Check if the given position is within this world element
    pub fn is_within(&self, position: Vec2) -> bool {
        return position.x >= self.final_coordinates.x - self.size.x / 2.0
            && position.x < self.final_coordinates.x + self.size.x / 2.0
            && position.y >= self.final_coordinates.y - self.size.y / 2.0
            && position.y < self.final_coordinates.y + self.size.y / 2.0
    }

    pub fn handle_window_resize(&mut self, new_window_size: &Vec2) {
        self.final_coordinates = self.position_type.shader_coordinates(&self.size, new_window_size);
    }

    // Put our element at the center of the given element (element_to_center_on)
    pub fn center_at(&mut self, element_to_center_on: &Self) {
        let width_difference = element_to_center_on.width() - self.width();
        let height_difference = element_to_center_on.height() - self.height();

        self.position_type = element_to_center_on.position_type.add_offset(width_difference / 2.0, height_difference / 2.0);
    }

    pub fn size(&self) -> &Vec2 { &self.size }
    pub fn width(&self) -> f32 { self.size.x }
    pub fn height(&self) -> f32 { self.size.y }
    pub fn z_index(&self) -> f32 { self.z_index }
}

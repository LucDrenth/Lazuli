use glam::Vec2;

/// World space data about size and positioning of a UI element
pub struct WorldElementData {
    size: Vec2, // given in world space (pixels)
    final_coordinates: Vec2, // the world space coordinates at which we actually render the center of the element, where (0,0) is the center of the screen
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

    pub fn size(&self) -> &Vec2 { &self.size }
    pub fn width(&self) -> f32 { self.size.x }
    pub fn height(&self) -> f32 { self.size.y }
    pub fn z_index(&self) -> f32 { self.z_index }
}

#[derive(Clone, Copy)]
pub enum Position {
    FixedCenter,
    FixedTop(f32),
    FixedBottom(f32),
    FixedLeft(f32),
    FixedRight(f32),
    FixedTopLeft(f32, f32),
}

impl Position {
    // Convert to world coordinates where (0, 0) is at the center of the screen
    pub fn shader_coordinates(&self, element_size: &Vec2, window_size: &Vec2) -> Vec2 {
        match self {
            Position::FixedTopLeft(x, y) => Vec2::new(
                *x - window_size.x / 2.0 + element_size.x / 2.0, 
                -(*y) + window_size.y / 2.0 - element_size.y / 2.0
            ),
            Position::FixedCenter => Vec2::ZERO,
            Position::FixedTop(top) => {
                Vec2::new(0.0, window_size.y / 2.0 - element_size.y / 2.0 - top)
            },
            Position::FixedBottom(bottom) => {
                Vec2::new(0.0, -window_size.y / 2.0 + element_size.y / 2.0 + bottom)
            },
            Position::FixedLeft(right) => {
                Vec2::new(-window_size.x / 2.0 + element_size.x / 2.0 + right, 0.0)
            },
            Position::FixedRight(left) => {
                Vec2::new(window_size.x / 2.0 - element_size.x / 2.0 - left, 0.0)
            },
        }
    }
}

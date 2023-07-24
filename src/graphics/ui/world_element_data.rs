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
    /// x is centered, y is measured from the top of the screen 
    FixedTop(f32),
    /// x is centered, y is measured from the bottom of the screen 
    FixedBottom(f32),
    /// x is measured from the left of the screen, y is centered
    FixedLeft(f32),
    /// x is measured from the right of the screen, y is centered
    FixedRight(f32),
    /// x is is measured from the left of the screen (param 1), y is measured from the top of the screen (param 2)
    FixedTopLeft(f32, f32), 
    /// x is is measured from the right of the screen (param 1), y is measured from the top of the screen (param 2)
    FixedTopRight(f32, f32),
    /// x is is measured from the left of the screen (param 1), y is measured from the bottom of the screen (param 2)
    FixedBottomLeft(f32, f32),
    /// x is is measured from the right of the screen (param 1), y is measured from the bottom of the screen (param 2)
    FixedBottomRight(f32, f32),
}

impl Position {
    // Convert to world coordinates where (0, 0) is at the center of the screen
    pub fn shader_coordinates(&self, element_size: &Vec2, window_size: &Vec2) -> Vec2 {
        match self {
            Position::FixedCenter => Vec2::ZERO,
            Position::FixedTop(top) => {
                Vec2::new(0.0, Self::y_for_fixed_top(element_size, window_size, top))
            },
            Position::FixedBottom(bottom) => {
                Vec2::new(0.0, Self::y_for_fixed_bottom(element_size, window_size, bottom))
            },
            Position::FixedLeft(left) => {
                Vec2::new(Self::x_for_fixed_left(element_size, window_size, left), 0.0)
            },
            Position::FixedRight(right) => {
                Vec2::new(Self::x_for_fixed_right(element_size, window_size, right), 0.0)
            },
            Position::FixedTopLeft(left, top) => Vec2::new(
                Self::x_for_fixed_left(element_size, window_size, left),
                Self::y_for_fixed_top(element_size, window_size, top),
            ),
            Position::FixedTopRight(right, top) => Vec2::new(
                Self::x_for_fixed_right(element_size, window_size, right),
                Self::y_for_fixed_top(element_size, window_size, top),
            ),
            Position::FixedBottomLeft(left, bottom) => Vec2::new(
                Self::x_for_fixed_left(element_size, window_size, left),
                Self::y_for_fixed_bottom(element_size, window_size, bottom),
            ),
            Position::FixedBottomRight(right, bottom) => Vec2::new(
                Self::x_for_fixed_right(element_size, window_size, right),
                Self::y_for_fixed_bottom(element_size, window_size, bottom),
            ),
        }
    }

    pub fn x_for_fixed_left(element_size: &Vec2, window_size: &Vec2, left: &f32,) -> f32 {
        -window_size.x / 2.0 + element_size.x / 2.0 + left
    }
    pub fn x_for_fixed_right(element_size: &Vec2, window_size: &Vec2, right: &f32) -> f32 {
        window_size.x / 2.0 - element_size.x / 2.0 - right
    }
    pub fn y_for_fixed_top(element_size: &Vec2, window_size: &Vec2, top: &f32) -> f32 {
        window_size.y / 2.0 - element_size.y / 2.0 - top
    }
    pub fn y_for_fixed_bottom(element_size: &Vec2, window_size: &Vec2, bottom: &f32) -> f32 {
        -window_size.y / 2.0 + element_size.y / 2.0 + bottom
    }
}

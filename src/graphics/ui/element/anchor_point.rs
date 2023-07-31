use glam::Vec2;

#[derive(Clone, Copy)]
pub enum AnchorPoint {
    /// * `x` - centered
    /// * `y` - centered
    Center,

    /// * `x` - centered
    /// * `y` - top of the element is `param_1` below the top of the anchor element
    TopInside(f32),
    /// * `x` - centered
    /// * `y` - bottom of the element is `param_1` apart from the top of the anchor element
    TopOutside(f32),
    /// * `x` - centered
    /// * `y` - bottom of the elements is `param_1` above the bottom of the anchor element
    BottomInside(f32),
    /// * `x` - centered
    /// * `y` - top of the element is `param_1` apart from the bottom of the anchor element
    BottomOutside(f32),
    /// * `x` - left side of the element is `param_1` to the right from the left side of the anchor element
    /// * `y` - centered
    LeftInside(f32),
    /// * `x` - right side of the element is `param_1` to the left of the left side of the anchor element
    /// * `y` - centered
    LeftOutside(f32),
    /// * `x` - right side of the element is `param_1` to the left from the right side of the anchor element
    /// * `y` - centered
    RightInside(f32),
    /// * `x` - left side of the element is `param_1` to the right of the right side of the anchor element
    /// * `y` - centered
    RightOutside(f32),

    /// * `x` - left side of the element is `param_1` to the left of the left side of the anchor element
    /// * `y` - top of the element is `param_2` below the top of the anchor element
    TopLeftInside(f32, f32),
    /// * `x` - right side of the element is `param_1` to the right of the left side of the anchor element
    /// * `y` - bottom of the element is `param_2` above the top of the anchor element
    TopLeftOutside(f32, f32),
    /// * `x` - right side of the element is `param_1` to the right of the right side of the anchor element
    /// * `y` - top of the element is `param_2` below the top of the anchor element
    TopRightInside(f32, f32),
    /// * `x` - left side of the element is `param_1` to the left of the right side of the anchor element
    /// * `y` - bottom of the element is `param_2` above the top of the anchor element
    TopRightOutside(f32, f32),

    /// * `x` - left side of the element is `param_1` to the left of the left side of the anchor element
    /// * `y` - bottom of the element is `param_2` above the bottom of the anchor element
    BottomLeftInside(f32, f32),
    /// * `x` - right side of the element is `param_1` to the right of the left side of the anchor element
    /// * `y` - top of the element is `param_2` below the bottom of the anchor element
    BottomLeftOutside(f32, f32),
    /// * `x` - right side of the element is `param_1` to the right of the right side of the anchor element
    /// * `y` - bottom of the element is `param_2` above the bottom of the anchor element
    BottomRightInside(f32, f32),
    /// * `x` - left side of the element is `param_1` to the left of the right side of the anchor element
    /// * `y` - top of the element is `param_2` below the bottom of the anchor element
    BottomRightOutside(f32, f32),
}

impl AnchorPoint {
    /// (0, 0) is the center of the screen
    pub fn to_coordinates(&self, element_size: Vec2, target_size: &Vec2, target_coordinates: &Vec2) -> Vec2 {
        match self {
            AnchorPoint::Center => target_coordinates.clone(),
            AnchorPoint::TopInside(y) => Vec2::new(
                target_coordinates.x,
                target_coordinates.y + target_size.y / 2.0 - element_size.y / 2.0 - y,
            ),
            AnchorPoint::TopOutside(y) => Vec2::new(
                target_coordinates.x,
                target_coordinates.y + target_size.y / 2.0 + element_size.y / 2.0 + y,
            ),
            AnchorPoint::BottomInside(y) => Vec2::new(
                target_coordinates.x,
                target_coordinates.y - target_size.y / 2.0 + element_size.y / 2.0 + y,
            ),
            AnchorPoint::BottomOutside(y) => Vec2::new(
                target_coordinates.x,
                target_coordinates.y - target_size.y / 2.0 - element_size.y / 2.0 - y,
            ),
            AnchorPoint::LeftInside(x) => Vec2::new(
                target_coordinates.x - target_size.x / 2.0 + element_size.x / 2.0 + x,
                target_coordinates.y,
            ),
            AnchorPoint::LeftOutside(x) => Vec2::new(
                target_coordinates.x - target_size.x / 2.0 - element_size.x / 2.0 - x, 
                target_coordinates.y,
            ),
            AnchorPoint::RightInside(x) => Vec2::new(
                target_coordinates.x + target_size.x / 2.0 - element_size.x / 2.0 - x,
                target_coordinates.y,
            ),
            AnchorPoint::RightOutside(x) => Vec2::new(
                target_coordinates.x + target_size.x / 2.0 + element_size.x / 2.0 + x, 
                target_coordinates.y,
            ),
            AnchorPoint::TopLeftInside(x, y) => Vec2::new(
                target_coordinates.x - target_size.x / 2.0 + element_size.x / 2.0 + x,
                target_coordinates.y + target_size.y / 2.0 - element_size.y / 2.0 - y,
            ),
            AnchorPoint::TopLeftOutside(x, y) => Vec2::new(
                target_coordinates.x - target_size.x / 2.0 - element_size.x / 2.0 - x, 
                target_coordinates.y + target_size.y / 2.0 + element_size.y / 2.0 + y,
            ),
            AnchorPoint::TopRightInside(x, y) => Vec2::new(
                target_coordinates.x + target_size.x / 2.0 - element_size.x / 2.0 - x,
                target_coordinates.y + target_size.y / 2.0 - element_size.y / 2.0 - y,
            ),
            AnchorPoint::TopRightOutside(x, y) =>  Vec2::new(
                target_coordinates.x + target_size.x / 2.0 + element_size.x / 2.0 + x, 
                target_coordinates.y + target_size.y / 2.0 + element_size.y / 2.0 + y,
            ),
            AnchorPoint::BottomLeftInside(x, y) => Vec2::new(
                target_coordinates.x - target_size.x / 2.0 + element_size.x / 2.0 + x,
                target_coordinates.y - target_size.y / 2.0 + element_size.y / 2.0 + y,
            ),
            AnchorPoint::BottomLeftOutside(x, y) => Vec2::new(
                target_coordinates.x - target_size.x / 2.0 - element_size.x / 2.0 - x, 
                target_coordinates.y - target_size.y / 2.0 - element_size.y / 2.0 - y,
            ),
            AnchorPoint::BottomRightInside(x, y) => Vec2::new(
                target_coordinates.x + target_size.x / 2.0 - element_size.x / 2.0 - x,
                target_coordinates.y - target_size.y / 2.0 + element_size.y / 2.0 + y,
            ),
            AnchorPoint::BottomRightOutside(x, y) => Vec2::new(
                target_coordinates.x + target_size.x / 2.0 + element_size.x / 2.0 + x, 
                target_coordinates.y - target_size.y / 2.0 - element_size.y / 2.0 - y,
            ),
        }
    }

    /// Translate from to the direction of the anchor point. 
    /// If anchor point is only on the x axis, amount_y will be ignored and vice versa.
    pub fn add_offset(&self, amount_x: f32, amount_y: f32) -> AnchorPoint {
        match self {
            AnchorPoint::Center => AnchorPoint::Center,
            AnchorPoint::TopInside(y) => AnchorPoint::TopInside(*y + amount_y),
            AnchorPoint::TopOutside(y) => AnchorPoint::TopOutside(*y + amount_y),
            AnchorPoint::BottomInside(y) => AnchorPoint::BottomInside(*y + amount_y),
            AnchorPoint::BottomOutside(y) => AnchorPoint::BottomOutside(*y + amount_y),
            AnchorPoint::LeftInside(x) => AnchorPoint::LeftInside(*x + amount_x),
            AnchorPoint::LeftOutside(x) => AnchorPoint::LeftOutside(*x + amount_x),
            AnchorPoint::RightInside(x) => AnchorPoint::RightInside(*x + amount_x),
            AnchorPoint::RightOutside(x) => AnchorPoint::RightOutside(*x + amount_x),

            AnchorPoint::TopLeftInside(x, y) => AnchorPoint::TopLeftInside(*x + amount_x, *y + amount_y),
            AnchorPoint::TopLeftOutside(x, y) => AnchorPoint::TopLeftOutside(*x + amount_x, *y + amount_y),
            AnchorPoint::TopRightInside(x, y) => AnchorPoint::TopRightInside(*x + amount_x, *y + amount_y),
            AnchorPoint::TopRightOutside(x, y) => AnchorPoint::TopRightOutside(*x + amount_x, *y + amount_y),
            AnchorPoint::BottomLeftInside(x, y) => AnchorPoint::BottomLeftInside(*x + amount_x, *y + amount_y),
            AnchorPoint::BottomLeftOutside(x, y) => AnchorPoint::BottomLeftOutside(*x + amount_x, *y + amount_y),
            AnchorPoint::BottomRightInside(x, y) => AnchorPoint::BottomRightInside(*x + amount_x, *y + amount_y),
            AnchorPoint::BottomRightOutside(x, y) => AnchorPoint::BottomRightOutside(*x + amount_x, *y + amount_y),
        }
    }
}

pub enum Padding {
    None,

    /// `param1` padding for each of the 4 sides
    Universal(f32),

    /// `param1` is vertical padding
    /// `param2` is horizontal padding
    HorizontalVertical(f32, f32),

    /// Clockwise parameters: Top, Right, Bottom, Left
    Individual(f32, f32, f32, f32),

    /// No horizontal padding
    Vertical(f32),

    /// No vertical padding
    Horizontal(f32),
}

impl Padding {
    pub fn top(&self) -> f32 {
        match self {
            Padding::None => 0.0,
            Padding::Universal(padding) => *padding,
            Padding::HorizontalVertical(_, padding_vertical) => *padding_vertical,
            Padding::Individual(padding_top, _, _, _) => *padding_top,
            Padding::Vertical(padding_vertical) => *padding_vertical,
            Padding::Horizontal(_) => 0.0,
        }
    }

    pub fn bottom(&self) -> f32 {
        match self {
            Padding::None => 0.0,
            Padding::Universal(padding) => *padding,
            Padding::HorizontalVertical(_, padding_vertical) => *padding_vertical,
            Padding::Individual(_, _, padding_bottom, _) => *padding_bottom,
            Padding::Vertical(padding_vertical) => *padding_vertical,
            Padding::Horizontal(_) => 0.0,
        }
    }

    pub fn left(&self) -> f32 {
        match self {
            Padding::None => 0.0,
            Padding::Universal(padding) => *padding,
            Padding::HorizontalVertical(padding_horizontal, _) => *padding_horizontal,
            Padding::Individual(_, _, _, padding_left) => *padding_left,
            Padding::Vertical(_) => 0.0,
            Padding::Horizontal(padding_horizontal) => *padding_horizontal,
        }
    }

    pub fn right(&self) -> f32 {
        match self {
            Padding::None => 0.0,
            Padding::Universal(padding) => *padding,
            Padding::HorizontalVertical(padding_horizontal, _) => *padding_horizontal,
            Padding::Individual(_, padding_right, _, _) => *padding_right,
            Padding::Vertical(_) => 0.0,
            Padding::Horizontal(padding_horizontal) => *padding_horizontal,
        }
    }

    pub fn horizontal(&self) -> f32 {
        self.left() + self.right()
    }

    pub fn vertical(&self) -> f32 {
        self.top() + self.bottom()
    }
}

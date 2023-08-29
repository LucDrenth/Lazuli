use crate::graphics::Color;

#[derive(Clone, Copy, Debug)]
pub struct BorderSize {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl BorderSize {
    pub fn zero() -> Self {
        Self { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }
    }

    pub fn set_universal(&mut self, size: f32) {
        self.top = size;
        self.right = size;
        self.bottom = size;
        self.left = size;
    }

    pub fn set_individual(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.left = left;
    }
}

#[derive(Clone, Debug)]
pub struct Border {
    pub color: Color,
    pub size: BorderSize,
}

use glam::Vec4;

use crate::graphics::Color;

#[derive(Clone, Debug)]
pub struct Border {
    pub color: Color,
    pub size: BorderSize,
    pub radius: BorderRadius,
}


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
        self.set_individual(size, size, size, size);
    }

    pub fn set_individual(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.left = left;
    }

    /// Return a tuple with the values in clockwise order
    pub fn tuple(&self) -> (f32, f32, f32, f32) {
        (self.top, self.right, self.bottom, self.left)
    }

    /// Return a vec4 with the values in clockwise order
    pub fn vec4(&self) -> Vec4 {
        Vec4::new(self.top, self.right, self.bottom, self.left)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl BorderRadius {
    pub fn zero() -> Self {
        Self { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
    }

    pub fn set_universal(&mut self, size: f32) {
        self.set_individual(size, size, size, size);
    }

    pub fn set_individual(&mut self, top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) {
        self.top_left = top_left;
        self.top_right = top_right;
        self.bottom_right = bottom_right;
        self.bottom_left = bottom_left;
    }

    /// x = top left
    /// y = top right
    /// z = bottom right
    /// w = bottom left
    pub fn to_vec(&self) -> Vec4{
        Vec4::new(self.top_left, self.top_right, self.bottom_right, self.bottom_left)
    }
}

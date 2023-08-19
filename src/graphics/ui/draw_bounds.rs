use glam::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct DrawBounds {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

/// glsl parameters can not be optional, so instead we will use an arbitratily high number so the element
/// will get drawn at a given bound
const SHADER_VALUE_FOR_NONE: f32 = 100_000.0;

impl DrawBounds {
    /// Parameters are screen coordinates where (0, 0) is at the center of the screen.
    /// * +y goes up
    /// * -y goes down
    /// * -x goes left
    /// * +x goes right
    pub fn some(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: Some(top),
            right: Some(right),
            bottom: Some(bottom),
            left: Some(left),
        }
    }

    pub fn none() -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }

    /// Convert coordinates to a tuple of (top, right, bottom, left) so that
    /// the coordinate (0, 0) is at the bottom left of the screen.
    pub fn for_shader(&self, screen_size: &Vec2, pixel_density: f32) -> (f32, f32, f32, f32) {        
        let top = match self.top {
            Some(amount) => amount + screen_size.y / 2.0,
            None => SHADER_VALUE_FOR_NONE,
        };
        let right = match self.right {
            Some(amount) => amount + screen_size.x / 2.0,
            None => SHADER_VALUE_FOR_NONE,
        };
        let bottom = match self.bottom {
            Some(amount) => amount + screen_size.y / 2.0,
            None => -SHADER_VALUE_FOR_NONE,
        };
        let left = match self.left {
            Some(amount) => amount + screen_size.x / 2.0,
            None => -SHADER_VALUE_FOR_NONE,
        };

        (top * pixel_density, right * pixel_density, bottom * pixel_density, left * pixel_density)
    }
}

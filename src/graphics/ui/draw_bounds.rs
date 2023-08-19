pub struct DrawBounds {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

impl DrawBounds {
    pub fn some(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: Some(top),
            right: Some(right),
            bottom: Some(bottom),
            left: Some(left),
        }
    }
}

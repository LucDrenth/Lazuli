use glam::Vec2;

use crate::graphics::texture::Texture;

pub struct MockTexture {
    pub size: Vec2,
}

impl Texture for MockTexture {
    fn activate(&self, _unit: usize) {}

    fn bind(&self) {}

    fn size(&self) -> Vec2 {
        self.size
    }

    fn width(&self) -> f32 {
        self.size.x
    }

    fn height(&self) -> f32 {
        self.size.y
    }
}

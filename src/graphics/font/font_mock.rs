use crate::{graphics::material::Material, ResourceId};

use super::{Bitmap, Font};

pub struct MockFont {
    pub atlas: Box<dyn Bitmap>,
    pub line_height: f32,
    pub space_size: f32,
    pub bitmap_spread: u8,
    pub material_id: ResourceId<Material>,
}

impl Font for MockFont {
    fn atlas(&self) -> &Box<dyn Bitmap> {
        &self.atlas
    }

    fn line_height(&self) -> f32 {
        self.line_height
    }

    fn space_size(&self) -> f32 {
        self.space_size
    }

    fn bitmap_spread(&self) -> u8 {
        self.bitmap_spread
    }

    fn get_material_id(&self) -> &ResourceId<Material> {
        &self.material_id
    }
}

use glam::Vec2;

use crate::{ResourceId, graphics::ui::UiElementId};

#[derive(Debug)]
pub struct AnchorElementData {
    pub id: ResourceId<UiElementId>,
    pub size: Vec2,
    pub coordinates: Vec2,
}

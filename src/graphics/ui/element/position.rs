use glam::Vec2;

use crate::graphics::ui::ElementRegistry;

use super::{AnchorPoint, AnchorElementData};

#[derive(Clone, Copy)]
pub enum Position {
    /// Exact positions
    Fixed(f32, f32),
    /// Anchored to a point on the screen
    ScreenAnchor(AnchorPoint),
    /// Anchored to another element. Second parameter is an intereface element_id
    ElementAnchor(AnchorPoint, u32),
}

impl Position {
    // Convert to world coordinates where (0, 0) is at the center of the screen
    pub fn to_coordinates(&self, element_size: Vec2, window_size: Vec2, anchor_element_data: Option<AnchorElementData>) -> Vec2 {
        match self {
            Position::Fixed(x, y) => Vec2::new(*x, *y),
            Position::ScreenAnchor(anchor_point) => {
                anchor_point.to_coordinates(element_size, &window_size, &Vec2::ZERO)
            },
            Position::ElementAnchor(anchor_point, _element_id) => {
                // We do not use the anchor element id (_element_id), since the given anchor_element_data is already from that id.
                // anchor_element_data.id == _element_id
                let anchor_element_data = anchor_element_data.unwrap();
                anchor_point.to_coordinates(
                    element_size, 
                    &anchor_element_data.size, 
                    &anchor_element_data.coordinates
                )
            },
        }
    }

    /// Translate from to the direction of the anchor point. 
    /// If anchor point is only on the x axis, amount_y will be ignored and vice versa.
    pub fn add_offset(&self, amount_x: f32, amount_y: f32) -> Position{
        match self {
            Position::Fixed(x, y) => Position::Fixed(*x + amount_x, *y + amount_y),
            Position::ScreenAnchor(anchor_point) => Position::ScreenAnchor(anchor_point.add_offset(amount_x, amount_y)),
            Position::ElementAnchor(anchor_point, element_id) => Position::ElementAnchor(anchor_point.add_offset(amount_x, amount_y), *element_id),
        }
    }

    /// Return size, coordinates
    pub fn get_anchor_element_id(&self) -> Option<u32> {
        match self {
            Position::Fixed(_, _) => None,
            Position::ScreenAnchor(_) => None,
            Position::ElementAnchor(_, element_id) => Some(*element_id),
        }
    }

    pub fn get_anchor_element_data(&self, element_registry: &ElementRegistry) -> Option<AnchorElementData> {
        match self {
            Position::Fixed(_, _) => None,
            Position::ScreenAnchor(_) => None,
            Position::ElementAnchor(_, element_id) => {
                Some(element_registry.get_anchor_data(*element_id).unwrap())
            },
        }
    }
}

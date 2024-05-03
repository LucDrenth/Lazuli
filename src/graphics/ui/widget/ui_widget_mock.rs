use glam::Vec2;

use crate::{graphics::ui::{bounds_2d::Bounds2d, ElementRegistry, Position, UiElementId, UiLayoutId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection}, ResourceId};

use super::UiWidget;

pub struct MockWidget {
    pub main_element_id: ResourceId<UiElementId>,
    pub direct_element_ids: Vec<ResourceId<UiElementId>>,
    pub direct_layout_ids: Vec<ResourceId<UiLayoutId>>,
    pub direct_widget_ids: Vec<ResourceId<UiWidgetId>>,
    pub z_index: f32,
}

impl Default for MockWidget {
    fn default() -> Self {
        Self { 
            main_element_id: ResourceId::new(0),
            direct_element_ids: vec![],
            direct_layout_ids: vec![],
            direct_widget_ids: vec![],
            z_index: 1.0,
        }
    }
}

impl UiWidget for MockWidget {
    fn get_direct_element_ids(&self) -> Vec<ResourceId<UiElementId>> {
        self.direct_element_ids.clone()
    }

    fn get_direct_layout_ids(&self) -> Vec<ResourceId<UiLayoutId>> {
        self.direct_layout_ids.clone()
    }

    fn get_direct_widget_ids(&self) -> Vec<ResourceId<UiWidgetId>> {
        self.direct_widget_ids.clone()
    }

    fn get_main_element_id(&self) -> ResourceId<UiElementId> {
        self.main_element_id.clone()
    }

    fn z_index(&self) -> f32 {
        self.z_index
    }

    fn set_z_index(&mut self, z_index: f32, _: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.z_index = z_index;

        UiUpdateTargets::default()
    }

    fn set_position(&self, _position: Position, _element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        UpdateTargetCollection::default()
    }

    fn set_draw_bounds(&self, _draw_bounds: Bounds2d, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d> {
        UiUpdateTargets::default()
    }

    fn set_width(&self, _width: f32, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        UiUpdateTargets::default()
    }

    fn set_height(&self, _height: f32, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        UiUpdateTargets::default()
    }

    fn set_size(&self, _size: glam::Vec2, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2> {
        UiUpdateTargets::default()
    }

    fn set_visibility(&mut self, _visible: bool, _element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        UiUpdateTargets::default()
    }

    fn is_debug(&self) -> bool {
        true
    }
}

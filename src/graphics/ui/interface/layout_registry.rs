use crate::{asset_manager::AssetManager, graphics::ui::{layout::LayoutBuilder, Layout, Position, UiLayoutId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection}, input::Input, ResourceId};

use super::{WidgetRegistry, ElementRegistry, element_list::generate_id};

struct LayoutEntry {
    layout: Box<dyn Layout>,
    id: ResourceId<UiLayoutId>
}

pub struct LayoutRegistry {
    layouts: Vec<LayoutEntry>,
}

impl LayoutRegistry {
    pub fn new() -> Self {
        Self { 
            layouts: vec![] 
        }
    }

    pub fn update(&mut self, 
        element_registry: &mut ElementRegistry, 
        widget_registry: &mut WidgetRegistry, 
        input: &Input,
        scroll_speed: f32,
    ) -> Vec<UpdateTargetCollection> {
        let mut update_targets = vec![];

        for i in 0..self.layouts.len() {
            update_targets.push(
                self.layouts[i].layout.update(
                    element_registry, widget_registry, self, input, scroll_speed
                )
            );
        }

        update_targets
    }

    pub fn create_layout(&mut self, 
        builder: &mut impl LayoutBuilder, 
        element_registry: &mut ElementRegistry, 
        widget_registry: &mut WidgetRegistry, 
        asset_manager: &mut dyn AssetManager
    ) -> Result<(ResourceId<UiLayoutId>, UpdateTargetCollection), String> {
        let (layout, update_targets) = builder.build(element_registry, widget_registry, self, asset_manager)?;
        let layout_id = self.add_layout(layout);

        Ok((layout_id, update_targets))
    }

    pub fn add_layout(&mut self, layout: Box<dyn Layout>) -> ResourceId<UiLayoutId> {
        let id: ResourceId<UiLayoutId> = ResourceId::new(generate_id());
        self.layouts.push(LayoutEntry { layout, id, });
        id.clone()
    }

    pub fn remove_layout(&mut self, layout_id: &ResourceId<UiLayoutId>) -> Option<Box<dyn Layout>> {
        for i in 0..self.layouts.len() {
            if self.layouts[i].id.equals(layout_id) {
                return Some(self.layouts.remove(i).layout)
            }
        }

        None
    }

    pub fn add_widget_to_layout(&mut self, 
        widget_id: &ResourceId<UiWidgetId>, 
        layout_id: &ResourceId<UiLayoutId>, 
        element_registry: &mut ElementRegistry, 
        widget_registry: &mut WidgetRegistry
    ) -> Result<UpdateTargetCollection, String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.add_widget(widget_id, element_registry, widget_registry, self)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }

    pub fn add_layout_to_layout(&mut self, 
        layout_id_to_add: &ResourceId<UiLayoutId>, 
        parent_layout_id: &ResourceId<UiLayoutId>, 
        element_registry: &mut ElementRegistry, 
        widget_registry: &mut WidgetRegistry
    ) -> Result<UpdateTargetCollection, String> {
        match self.get_mut_layout(parent_layout_id) {
            Some(layout) => Ok(layout.add_layout(layout_id_to_add, element_registry, widget_registry, self)),
            None => Err(Self::layout_not_found(parent_layout_id)),
        }
    }

    pub fn set_layout_z_index(&mut self, 
        layout_id: &ResourceId<UiLayoutId>, 
        z_index: f32, 
        element_registry: &mut ElementRegistry, 
    ) -> Result<UiUpdateTargets<f32>, String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.set_z_index(z_index, element_registry)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }

    pub fn set_layout_width(&mut self, 
        layout_id: &ResourceId<UiLayoutId>, 
        width: f32, 
        element_registry: &mut ElementRegistry, 
    ) -> Result<UpdateTargetCollection, String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.set_width(width, element_registry)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }

    pub fn set_layout_visibility(&mut self, 
        layout_id: &ResourceId<UiLayoutId>, 
        visible: bool, 
        element_registry: &mut ElementRegistry, 
    ) -> Result<UiUpdateTargets<bool>, String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.set_visibility(visible, element_registry)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }

    pub fn set_layout_position(&mut self, 
        layout_id: &ResourceId<UiLayoutId>, 
        position: Position, 
        element_registry: &mut ElementRegistry, 
    ) -> Result<UpdateTargetCollection, String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.set_position(position, element_registry)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }

    fn layout_not_found(layout_id: &ResourceId<UiLayoutId>) -> String {
        format!("Layout with id {:?} not found", layout_id)
    }

    pub fn get_layout(&self, layout_id: &ResourceId<UiLayoutId>) -> Option<&Box<dyn Layout>> {
        for entry in self.layouts.iter() {
            if entry.id.equals(&layout_id) {
                return Some(&entry.layout)
            }
        }

        None
    }
    
    pub fn get_mut_layout(&mut self, layout_id: &ResourceId<UiLayoutId>) -> Option<&mut Box<dyn Layout>> {
        for entry in self.layouts.iter_mut() {
            if entry.id.equals(&layout_id) {
                return Some(&mut entry.layout)
            }
        }

        None
    }

    pub fn handle_window_resize(&mut self, element_registry: &ElementRegistry) -> Vec<UpdateTargetCollection> {
        let mut targets: Vec<UpdateTargetCollection> = vec![];

        for entry in self.layouts.iter_mut() {
            targets.push( entry.layout.update_draw_bounds(element_registry) );
        }

        targets
    }

    pub fn update_layout_draw_bounds(&mut self, 
        layout_id: &ResourceId<UiLayoutId>, 
        element_registry: &ElementRegistry
    ) -> Result<UpdateTargetCollection, String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.update_draw_bounds(element_registry)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }
}

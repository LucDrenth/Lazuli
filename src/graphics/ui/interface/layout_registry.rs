use crate::{graphics::ui::{Layout, layout::LayoutBuilder, UiLayoutId, UiWidgetId}, input::Input, ResourceId, asset_manager::AssetManager};

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
    ) {
        for entry in self.layouts.iter_mut() {
            entry.layout.update(element_registry, widget_registry, input, scroll_speed);
        }
    }

    pub fn create_layout(&mut self, 
        builder: &mut impl LayoutBuilder, 
        element_registry: &mut ElementRegistry, 
        widget_registry: &mut WidgetRegistry, 
        asset_manager: &mut AssetManager
    ) -> Result<ResourceId<UiLayoutId>, String> {
        let layout = builder.build(element_registry, widget_registry, asset_manager)?;
        Ok(self.add_layout(layout))
    }

    pub fn add_layout(&mut self, layout: Box<dyn Layout>) -> ResourceId<UiLayoutId> {
        let id: ResourceId<UiLayoutId> = ResourceId::new(generate_id());
        self.layouts.push(LayoutEntry { layout, id, });
        id.clone()
    }

    pub fn add_widget_to_layout(&mut self, 
        widget_id: &ResourceId<UiWidgetId>, 
        layout_id: &ResourceId<UiLayoutId>, 
        element_registry: &mut ElementRegistry, 
        widget_registry: &mut WidgetRegistry
    ) -> Result<(), String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.add_widget(widget_id, element_registry, widget_registry)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }

    pub fn set_layout_z_index(&mut self, 
        layout_id: &ResourceId<UiLayoutId>, 
        z_index: f32, 
        element_registry: &mut ElementRegistry, 
        widget_registry: &mut WidgetRegistry
    ) -> Result<(), String> {
        match self.get_mut_layout(layout_id) {
            Some(layout) => Ok(layout.set_z_index(z_index, element_registry, widget_registry)),
            None => Err(Self::layout_not_found(layout_id)),
        }
    }

    fn layout_not_found(layout_id: &ResourceId<UiLayoutId>) -> String {
        format!("Layout with id {:?} not found", layout_id)
    }

    pub fn get_layout(&mut self, layout_id: &ResourceId<UiLayoutId>) -> Option<&Box<dyn Layout>> {
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
}

use std::fmt::Debug;

use crate::{graphics::ui::{widget::UiWidget, UiWidgetId}, log, ResourceId};

use super::element_list::generate_id;

pub struct WidgetEntry<T: UiWidget, U: Debug + Clone> {
    pub widget: T,
    pub id: ResourceId<UiWidgetId>,
    pub update_result: U,
}

impl <T: UiWidget, U: Debug + Clone> WidgetEntry<T, U> {
    fn new(widget: T, default_update_result: U) -> Self {
        Self {
            widget: widget,
            id: ResourceId::new(generate_id()),
            update_result: default_update_result,
        }
    }
}

pub struct WidgetList<T: UiWidget, U: Debug + Clone> {
    pub entries: Vec<WidgetEntry<T, U>>,
    pub default_update_result: U,
}

impl <T: UiWidget, U: Debug + Clone> WidgetList<T, U> {
    pub fn new(default_update_result: U) -> Self {
        Self { entries: vec![], default_update_result }
    }

    pub fn push(&mut self, widget: T) -> ResourceId<UiWidgetId> {
        self.entries.push(WidgetEntry::new(widget, self.default_update_result.clone()));
        let id = self.entries.last().unwrap().id.duplicate();

        self.sort();

        id
    }

    pub fn get_widget(&self, widget_id: &ResourceId<UiWidgetId>) -> Option<&T>  {
        for entry in self.entries.iter() {
            if entry.id.equals(&widget_id) {
                return Some(&entry.widget);
            }
        }

        None
    }

    pub fn get_mut_widget(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<&mut T> {
        for entry in self.entries.iter_mut() {
            if entry.id.equals(&widget_id) {
                return Some(&mut entry.widget);
            }
        }

        None
    }

    /// Returns the update result of the given widget, or default if the widget was not found
    pub fn get_update_result(&self, widget_id: &ResourceId<UiWidgetId>) -> U {
        for entry in self.entries.iter() {
            if entry.id.equals(&widget_id) {
                return entry.update_result.clone();
            }
        }

        log::engine_warn(format!(
            "WidgetList.get_update_result returning default {:?} because widget with id {} was not found", 
            self.default_update_result, widget_id.id())
        );

        self.default_update_result.clone()
    }

    // Returns the removed element, or None if it was not found
    pub fn remove(&mut self, widget_id: &ResourceId<UiWidgetId>) -> Option<T> {
        for i in 0..self.entries.len() {
            if self.entries[i].id.equals(&widget_id) {
                return Some(self.entries.remove(i).widget);
            }
        }

        None
    }

    // Sort elements so that the elements with the lowest z-index are at the start of the list
    fn sort(&mut self) {
        self.entries.sort_by(|a, b| b.widget.z_index().total_cmp(&a.widget.z_index()));
    }
}

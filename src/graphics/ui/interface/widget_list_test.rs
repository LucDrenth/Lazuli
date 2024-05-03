use crate::{graphics::ui::widget::ui_widget_mock::MockWidget, ResourceId};
use super::widget_list::WidgetList;

#[test]
/// Test both get_widget and
fn test_get_widget() {
    use crate::{graphics::ui::UiWidgetId, ResourceId};

    let mut widget_list: WidgetList<MockWidget, ()> = WidgetList::new(());

    let widget_id = widget_list.push(MockWidget::default());
    let non_existing_widget_id: ResourceId<UiWidgetId> = ResourceId::new(widget_id.id() + 1);

    assert_eq!(true, widget_list.get_widget(&widget_id).is_some());
    assert_eq!(true, widget_list.get_widget(&non_existing_widget_id).is_none());

    assert_eq!(true, widget_list.get_mut_widget(&widget_id).is_some());
    assert_eq!(true, widget_list.get_mut_widget(&non_existing_widget_id).is_none());
}

#[test]
fn test_get_update_result() {
    let default_update_result = 1;
    let custom_update_result = 2;

    let mut widget_list: WidgetList<MockWidget, usize> = WidgetList::new(default_update_result);

    let widget_id = widget_list.push(MockWidget::default());
    widget_list.entries[0].update_result = custom_update_result;
    let non_existing_widget_id = ResourceId::new(widget_id.id() + 1);

    assert_eq!(widget_list.get_update_result(&widget_id), custom_update_result);
    assert_eq!(widget_list.get_update_result(&non_existing_widget_id), default_update_result);
}

#[test]
/// Test wether the widgets are properly sorted by z index after pushing a widget
fn test_sort_after_push() {
    let mut widget_list: WidgetList<MockWidget, ()> = WidgetList::new(());

    for z_index in vec![1.0, 0.5, 15.34, 2.5, 20.0] {
        let mut widget = MockWidget::default();
        widget.z_index = z_index;
        widget_list.push(widget);
    }

    let mut z_index: f32 = f32::MAX;
    for entry in widget_list.entries {
        assert!(entry.widget.z_index < z_index);
        z_index = entry.widget.z_index;
    }
}

// We can not test any other methods since we would need to pass a ElementRegistry or WidgetRegistry to
// most methods. This would complicate things more than it would help.

use crate::{graphics::ui::layout::MockLayout, ResourceId};

use super::LayoutRegistry;

#[test]
/// Test get_layout and get_mut_layout
fn test_get_layout() {
    let mut layout_registry = LayoutRegistry::new();

    let layout_id = layout_registry.add_layout(Box::new(MockLayout::default()));
    let non_existing_layout_id = ResourceId::new(layout_id.id() + 1);

    assert!(layout_registry.get_layout(&layout_id).is_some());
    assert!(layout_registry.get_layout(&non_existing_layout_id).is_none());

    assert!(layout_registry.get_mut_layout(&layout_id).is_some());
    assert!(layout_registry.get_mut_layout(&non_existing_layout_id).is_none());
}

#[test]
fn test_remove_layout() {
    let mut layout_registry = LayoutRegistry::new();

    let layout_id = layout_registry.add_layout(Box::new(MockLayout::default()));
    let non_existing_layout_id = ResourceId::new(layout_id.id() + 1);

    assert!(layout_registry.remove_layout(&non_existing_layout_id).is_none());
    assert!(layout_registry.remove_layout(&layout_id).is_some());
    assert!(layout_registry.remove_layout(&layout_id).is_none());
}

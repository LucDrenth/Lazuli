/// These are used for ResourceId, e.g. `ResourceId<UiWidgetId>`. The main purpouse of using
/// these is to distinguish `UiWidget` ids from `UiElement` ids. 
/// 
/// If we want to use each individual widget or element struct as `ResourceId<T>` to distinguish 
/// between widget/element types, we would have to do `Vec<ResourceId<&dyn UiWidget|UiElement>>`
/// if we want to keep a list of widget ids (such as in Layouts). This means we need to introduce
/// lifetimes, which causes a whole lot of complexity which I am currently not ready for.
//

#[derive(Debug, Clone, Copy)]
pub struct UiWidgetId;

#[derive(Debug, Clone, Copy)]
pub struct UiElementId;

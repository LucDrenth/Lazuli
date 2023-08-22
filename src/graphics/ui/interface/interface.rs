use glam::Vec2;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, asset_manager::AssetManager, input::Input, graphics::{ui::{widget::{SliderBuilder, SliderUpdateResult, ButtonBuilder, DropdownBuilder}, Position, draw_bounds::DrawBounds, UiWidgetId, UiElementId}, font::{Font, PlainBitmapBuilder}, Color}, ResourceId};

use super::{ElementRegistry, widget_registry::WidgetRegistry};

pub struct Interface {
    element_registry: ElementRegistry,
    window_resize_listener: EventReader<WindowResizeEvent>,
    widget_registry: WidgetRegistry,
    size: Vec2,
    scroll_speed: f32,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem, window_size: Vec2, pixel_density: f32) -> Self {
        Self {
            element_registry: ElementRegistry::new(window_size, pixel_density),
            widget_registry: WidgetRegistry::new(),
            window_resize_listener: event_system.register::<WindowResizeEvent>(),
            size: window_size,
            scroll_speed: 0.2,
        }
    }

    pub fn update(&mut self, asset_manager: &mut AssetManager, input: &Input) {
        // We update widget_registry before element_registry so that we won't activate any mouse_up
        // events while we were still dragging an element (which gets reset by element_registry.update)
        self.widget_registry.update(input, &mut self.element_registry, asset_manager);
        self.element_registry.update(asset_manager, input);
        
        self.window_resize_listener.read().last().map(|e| {
            let window_size = Vec2::new(e.width as f32, e.height as f32);
            self.element_registry.handle_window_resize(window_size, asset_manager);
        });
    }

    pub fn draw(&self, asset_manager: &mut AssetManager) {
        self.element_registry.draw(asset_manager);
    }

    pub fn element_registry(&self) -> &ElementRegistry { &self.element_registry }
    pub fn mut_element_registry(&mut self) -> &mut ElementRegistry { &mut self.element_registry }
    pub fn widget_registry(&self) ->  &WidgetRegistry { &self.widget_registry }
    pub fn mut_widget_registry(&mut self) -> &mut WidgetRegistry { &mut self.widget_registry }

    // UiWidget functions
    pub fn get_widget_main_element_id(&self, widget_id: &ResourceId<UiWidgetId>) -> Option<ResourceId<UiElementId>> {
        match self.widget_registry.get_widget_by_id(widget_id) {
            Some(widget) => Some(widget.get_main_element_id()),
            None => None,
        }
    }
    pub fn show_widget(&mut self, widget_id: &ResourceId<UiWidgetId>) {
        self.widget_registry.get_widget_by_id(widget_id).unwrap().show(&mut self.element_registry);
    }
    pub fn hide_widget(&mut self, widget_id: &ResourceId<UiWidgetId>) {
        self.widget_registry.get_widget_by_id(widget_id).unwrap().hide(&mut self.element_registry);
    }
    pub fn get_widget_size(&self, widget_id: &ResourceId<UiWidgetId>) -> Result<Vec2, String> {
        let main_element_id = self.get_widget_main_element_id(widget_id).unwrap();
        self.element_registry.get_element_size(&main_element_id)
    }
    pub fn get_widget_screen_position(&self, widget_id: &ResourceId<UiWidgetId>) -> Result<Vec2, String> {
        let main_element_id = self.get_widget_main_element_id(widget_id).unwrap();
        self.element_registry.get_element_screen_position(&main_element_id)
    }
    pub fn get_widget_position_transform(&self, widget_id: &ResourceId<UiWidgetId>) -> Result<Vec2, String> {
        let main_element_id = self.get_widget_main_element_id(widget_id).unwrap();
        self.element_registry.get_element_position_transform(&main_element_id)
    }
    pub fn set_widget_position(&mut self, widget_id: &ResourceId<UiWidgetId>, position: Position) {
        self.widget_registry.get_widget_by_id(widget_id).unwrap().set_position(position, &mut self.element_registry)
    }
    pub fn set_widget_z_index(&mut self, widget_id: &ResourceId<UiWidgetId>, z_index: f32) {
        self.widget_registry.get_mut_widget_by_id(widget_id).unwrap().set_z_index(z_index, &mut self.element_registry);
    }
    pub fn set_widget_draw_bounds(&mut self, widget_id: &ResourceId<UiWidgetId>, draw_bounds: DrawBounds) {
        self.widget_registry.get_mut_widget_by_id(widget_id).unwrap().set_draw_bounds(draw_bounds, &mut self.element_registry);
    }
    pub fn set_widget_width(&mut self, widget_id: &ResourceId<UiWidgetId>, width: f32) {
        self.widget_registry.get_widget_by_id(widget_id).unwrap().set_width(width, &mut self.element_registry);
    }
    pub fn set_widget_height(&mut self, widget_id: &ResourceId<UiWidgetId>, height: f32) {
        self.widget_registry.get_widget_by_id(widget_id).unwrap().set_height(height, &mut self.element_registry);
    }
    pub fn set_widget_size(&mut self, widget_id: &ResourceId<UiWidgetId>, size: Vec2) {
        self.widget_registry.get_widget_by_id(widget_id).unwrap().set_size(size, &mut self.element_registry);
    }

    // button specific functions
    pub fn add_button(&mut self, label: impl Into<String>, builder: &ButtonBuilder, asset_manager: &mut AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        self.widget_registry.add_button(label, builder, &mut self.element_registry, asset_manager)
    }
    pub fn is_button_clicked(&self, button_id: &ResourceId<UiWidgetId>) -> bool {
        self.widget_registry.is_button_clicked(button_id)
    }
    pub fn set_button_background_color(&mut self, color: Color, button_id: &ResourceId<UiWidgetId>) -> Result<(), String> {
        self.widget_registry.set_button_background_color(color, button_id, &mut self.element_registry)
    }
    pub fn set_button_text_color(&mut self, color: Color, button_id: &ResourceId<UiWidgetId>) -> Result<(), String> {
        self.widget_registry.set_button_text_color(color, button_id, &mut self.element_registry)
    }

    // slider specific functions
    pub fn add_slider(&mut self, builder: SliderBuilder, asset_manager: &mut AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        self.widget_registry.add_slider(builder, &mut self.element_registry, asset_manager)
    }
    pub fn slider_update_result(&self, slider_id: &ResourceId<UiWidgetId>) -> Option<SliderUpdateResult> {
        self.widget_registry.slider_update_result(slider_id)
    }
    pub fn set_slider_value(&mut self, value: f32, slider_id: &ResourceId<UiWidgetId>, asset_manager: &mut AssetManager) {
        self.widget_registry.set_slider_value(value, slider_id, &mut self.element_registry, asset_manager);
    }

    // dropdown specific functions
    pub fn add_dropdown(&mut self, builder: DropdownBuilder<u32>, asset_manager: &mut AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        self.widget_registry.add_dropdown(builder, &mut self.element_registry, asset_manager)
    }
    pub fn dropdown_update_result(&self, dropdown_id: &ResourceId<UiWidgetId>) -> Option<u32> {
        self.widget_registry.dropdown_update_result(dropdown_id)
    }

    pub fn scroll_speed(&self) -> f32 {
        self.scroll_speed
    }
}

// TODO make these configurable
pub fn default_text_color() -> Color { Color::Rgb(239, 239, 239) }
pub fn default_element_background_color() -> Color { Color::Rgb(56, 56, 56) }
pub fn default_font_size() -> f32 {
    14.0
}
pub fn default_font(asset_manager: &mut AssetManager) -> Result<ResourceId<Font>, String> {
    asset_manager.load_font(PlainBitmapBuilder::new()
        .with_font_file_path("./assets/fonts/roboto.ttf".to_string())
        .with_font_size(50.0)
    , None)
}

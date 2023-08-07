use glam::Vec2;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, asset_manager::{AssetManager, AssetId}, input::Input, graphics::{ui::widget::{SliderBuilder, SliderUpdateResult, ButtonBuilder}, font::{Font, PlainBitmapBuilder}, Color}};

use super::{ElementRegistry, widget_registry::WidgetRegistry};

pub struct Interface {
    element_registry: ElementRegistry,
    window_resize_listener: EventReader<WindowResizeEvent>,
    widget_registry: WidgetRegistry,
    size: Vec2,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem, window_size: Vec2) -> Self {
        Self {
            element_registry: ElementRegistry::new(window_size),
            widget_registry: WidgetRegistry::new(),
            window_resize_listener: event_system.register::<WindowResizeEvent>(),
            size: window_size,
        }
    }

    pub fn update(&mut self, asset_manager: &mut AssetManager, input: &Input) {
        self.element_registry.update(asset_manager, input);
        self.widget_registry.update(input, &mut self.element_registry, asset_manager);
        
        self.window_resize_listener.read().last().map(|e| {
            let window_size = Vec2::new(e.width as f32, e.height as f32);
            self.element_registry.handle_window_resize(window_size, asset_manager);
        });
    }

    pub fn draw(&self, asset_manager: &mut AssetManager) {
        self.element_registry.draw(asset_manager);
    }

    pub fn add_slider(&mut self, builder: SliderBuilder, asset_manager: &mut AssetManager) -> Result<u32, String> {
        self.widget_registry.add_slider(builder, &mut self.element_registry, asset_manager)
    }
    pub fn slider_update_result(&self, slider_id: u32) -> Option<SliderUpdateResult> {
        self.widget_registry.slider_update_result(slider_id)
    }
    pub fn slider_anchor_element_id(&self, slider_id: u32) -> Option<u32> {
        self.widget_registry.slider_anchor_element_id(slider_id)
    }
    pub fn set_slider_value(&mut self, value: f32, slider_id: u32, asset_manager: &mut AssetManager) {
        self.widget_registry.set_slider_value(value, slider_id, &mut self.element_registry, asset_manager);
    }
    pub fn show_slider(&mut self, slider_id: u32) {
        self.widget_registry.show_slider(slider_id, &mut self.element_registry);
    }
    pub fn hide_slider(&mut self, slider_id: u32) {
        self.widget_registry.hide_slider(slider_id, &mut self.element_registry);
    }

    pub fn add_button(&mut self, label: String, builder: ButtonBuilder, asset_manager: &mut AssetManager) -> Result<u32, String> {
        self.widget_registry.add_button(label, builder, &mut self.element_registry, asset_manager)
    }
    pub fn is_button_clicked(&self, button_id: u32) -> bool {
        self.widget_registry.is_button_clicked(button_id)
    }
    pub fn button_anchor_element_id(&self, button_id: u32) -> Option<u32> {
        self.widget_registry.button_anchor_element_id(button_id)
    }
    pub fn show_button(&mut self, button_id: u32) {
        self.widget_registry.show_button(button_id, &mut self.element_registry);
    }
    pub fn hide_button(&mut self, button_id: u32) {
        self.widget_registry.hide_button(button_id, &mut self.element_registry);
    }

    pub fn mut_element_registry(&mut self) -> &mut ElementRegistry { &mut self.element_registry }
}

// TODO make these configurable
pub fn default_text_color() -> Color { Color::Rgb(239, 239, 239) }
pub fn default_element_background_color() -> Color { Color::Rgb(56, 56, 56) }
pub fn default_font_size() -> f32 {
    14.0
}
pub fn default_font(asset_manager: &mut AssetManager) -> Result<AssetId<Font>, String> {
    asset_manager.load_font(PlainBitmapBuilder::new()
        .with_font_file_path("./assets/fonts/roboto.ttf".to_string())
        .with_font_size(50.0)
    , None)
}

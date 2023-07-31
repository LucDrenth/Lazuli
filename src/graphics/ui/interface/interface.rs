use glam::Vec2;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem}, asset_registry::AssetRegistry, input::Input, graphics::ui::widget::{SliderBuilder, SliderUpdateResult}};

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

    pub fn update(&mut self, asset_registry: &mut AssetRegistry, input: &Input) {
        self.element_registry.update(asset_registry, input);
        self.widget_registry.update(input, &mut self.element_registry, asset_registry);
        
        self.window_resize_listener.read().last().map(|e| {
            let window_size = Vec2::new(e.width as f32, e.height as f32);
            self.element_registry.handle_window_resize(window_size, asset_registry);
        });
    }

    pub fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.element_registry.draw(asset_registry);
    }

    pub fn add_slider(&mut self, builder: SliderBuilder, asset_registry: &mut AssetRegistry) -> Result<u32, String> {
        self.widget_registry.add_slider(builder, &mut self.element_registry, asset_registry)
    }
    pub fn slider_update_result(&self, slider_id: u32) -> Option<SliderUpdateResult> {
        self.widget_registry.slider_update_result(slider_id)
    }
    pub fn slider_anchor_element_id(&self, slider_id: u32) -> Option<u32> {
        self.widget_registry.slider_anchor_element_id(slider_id)
    }
    pub fn set_slider_value(&mut self, value: f32, slider_id: u32, asset_registry: &mut AssetRegistry) {
        self.widget_registry.set_slider_value(value, slider_id, &mut self.element_registry, asset_registry);
    }

    pub fn mut_element_registry(&mut self) -> &mut ElementRegistry { &mut self.element_registry }
}

pub fn default_text_color() -> (u8, u8, u8) { (239, 239, 239) }
pub fn default_element_background_color() -> (u8, u8, u8) { (56, 56, 56) }

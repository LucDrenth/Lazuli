use glam::Vec2;

use crate::{event::{EventReader, WindowResizeEvent, EventSystem, PixelDensityChangeEvent}, asset_manager::AssetManager, input::Input, graphics::{ui::{widget::{SliderBuilder, SliderUpdateResult, ButtonBuilder, DropdownBuilder, IconBuilder}, Position, bounds_2d::Bounds2d, UiWidgetId, UiElementId, UiLayoutId, layout::LayoutBuilder, UiUpdateTargets, UpdateTargetCollection}, font::{Font, PlainBitmapBuilder}, Color}, ResourceId, log};

use super::{ElementRegistry, widget_registry::{WidgetRegistry, WidgetRegistryUdpateResult}, layout_registry::LayoutRegistry};

pub struct Interface {
    element_registry: ElementRegistry,
    widget_registry: WidgetRegistry,
    layout_registry: LayoutRegistry,
    size: Vec2,
    scroll_speed: f32,

    window_resize_listener: EventReader<WindowResizeEvent>,
    pixel_density_change_event: EventReader<PixelDensityChangeEvent>,
}

impl Interface {
    pub fn new(event_system: &mut EventSystem, window_size: Vec2, pixel_density: f32) -> Self {
        Self {
            element_registry: ElementRegistry::new(window_size, pixel_density),
            widget_registry: WidgetRegistry::new(),
            layout_registry: LayoutRegistry::new(),
            size: window_size,
            scroll_speed: 0.2,

            window_resize_listener: event_system.register(),
            pixel_density_change_event: event_system.register(),
        }
    }

    pub fn update(&mut self, asset_manager: &mut AssetManager, input: &Input) {
        // We update widget_registry before element_registry so that we won't activate any mouse_up
        // events while we were still dragging an element (which gets reset by element_registry.update)
        let widget_registry_update_result = &self.widget_registry.update(input, &mut self.element_registry, asset_manager);
        self.handle_widget_registry_update_result(&widget_registry_update_result, asset_manager);
        self.element_registry.update(asset_manager, input);
        
        self.window_resize_listener.read().last().map(|e| {
            let window_size = Vec2::new(e.width as f32, e.height as f32);
            self.element_registry.handle_window_resize(window_size, asset_manager);

            // TODO update layout elements their draw bounds. See HelloUi scene, where the
            // layout elements get buggy after window resize.
        });

        self.pixel_density_change_event.read().last().map(|e| {
            self.element_registry.set_pixel_density(e.pixel_density);
        });

        self.layout_registry.update(&mut self.element_registry, &mut self.widget_registry, input, self.scroll_speed);
    }

    pub fn handle_widget_registry_update_result(&mut self, update_result: &WidgetRegistryUdpateResult, asset_manager: &mut AssetManager) {
        self.handle_ui_update_targets_visibility(update_result.update_targets_visibility.clone());

        for target in &update_result.buttons_to_change_text {
            let element_id = self.widget_registry.get_button(&target.widget_id).unwrap().text_element_id();

            match self.element_registry.set_text(&element_id, &target.data, asset_manager) {
                Ok(()) => (),
                Err(err) => {
                    log::engine_err(format!(
                        "interface failed to update text for element {:?} from widget {:?} through widget update result: {}",
                        element_id, target.widget_id, err
                    ));
                },
            }
        }

        for target in &update_result.widgets_to_set_main_element_custom_shader_value_f32 {
            let main_element_id = self.get_widget_main_element_id(&target.widget_id).unwrap();
            self.element_registry.get_mut_element_custom_shader_values(&main_element_id).unwrap().set_f32(target.data.0.clone(), target.data.1);
        }
    }

    pub fn draw(&self, asset_manager: &mut AssetManager) {
        self.element_registry.draw(asset_manager);
    }

    pub fn element_registry(&self) -> &ElementRegistry { &self.element_registry }
    pub fn mut_element_registry(&mut self) -> &mut ElementRegistry { &mut self.element_registry }
    pub fn size(&mut self) -> Vec2 { self.size }

    // UiWidget functions
    pub fn get_widget_main_element_id(&self, widget_id: &ResourceId<UiWidgetId>) -> Option<ResourceId<UiElementId>> {
        self.widget_registry.get_widget_main_element_id(widget_id)
    }
    pub fn get_widget_size(&self, widget_id: &ResourceId<UiWidgetId>) -> Result<Vec2, String> {
        self.widget_registry.get_widget_size(widget_id, &self.element_registry)
    }
    pub fn get_widget_screen_position(&self, widget_id: &ResourceId<UiWidgetId>) -> Result<Vec2, String> {
        self.widget_registry.get_widget_screen_position(widget_id, &self.element_registry)
    }
    pub fn get_widget_position_transform(&self, widget_id: &ResourceId<UiWidgetId>) -> Result<Vec2, String> {
        self.widget_registry.get_widget_position_transform(widget_id, &self.element_registry)
    }
    pub fn set_widget_position(&mut self, widget_id: &ResourceId<UiWidgetId>, position: Position) {
        self.handle_ui_update_targets_position(UiUpdateTargets::from_widget_id(widget_id.clone(), position));
    }
    pub fn set_widget_z_index(&mut self, widget_id: &ResourceId<UiWidgetId>, z_index: f32) {
        self.handle_ui_update_targets_z_index(UiUpdateTargets::from_widget_id(widget_id.clone(), z_index));
    }
    pub fn set_widget_draw_bounds(&mut self, widget_id: &ResourceId<UiWidgetId>, draw_bounds: Bounds2d) {
        self.handle_ui_update_targets_draw_bounds(UiUpdateTargets::from_widget_id(widget_id.clone(), draw_bounds));
    }
    pub fn set_widget_width(&mut self, widget_id: &ResourceId<UiWidgetId>, width: f32) {
        self.handle_ui_update_targets_width(UiUpdateTargets::from_widget_id(widget_id.clone(), width));
    }
    pub fn set_widget_height(&mut self, widget_id: &ResourceId<UiWidgetId>, height: f32) {
        self.handle_ui_update_targets_height(UiUpdateTargets::from_widget_id(widget_id.clone(), height));
    }
    pub fn set_widget_size(&mut self, widget_id: &ResourceId<UiWidgetId>, size: Vec2) {
        self.handle_ui_update_targets_size(UiUpdateTargets::from_widget_id(widget_id.clone(), size));
    }
    pub fn set_widget_visibility(&mut self, widget_id: &ResourceId<UiWidgetId>, visible: bool) {
        self.handle_ui_update_targets_visibility(UiUpdateTargets::from_widget_id(widget_id.clone(), visible));
    }

    // button specific functions
    pub fn create_button(&mut self, label: impl Into<String>, builder: &ButtonBuilder, asset_manager: &mut AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        self.widget_registry.create_button(label, builder, &mut self.element_registry, asset_manager)
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
    pub fn create_slider(&mut self, builder: &SliderBuilder, asset_manager: &mut AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        self.widget_registry.create_slider(builder, &mut self.element_registry, asset_manager)
    }
    pub fn slider_update_result(&self, slider_id: &ResourceId<UiWidgetId>) -> Option<SliderUpdateResult> {
        self.widget_registry.slider_update_result(slider_id)
    }
    pub fn set_slider_value(&mut self, value: f32, slider_id: &ResourceId<UiWidgetId>, asset_manager: &mut AssetManager) {
        self.widget_registry.set_slider_value(value, slider_id, &mut self.element_registry, asset_manager);
    }

    // dropdown specific functions
    pub fn create_dropdown(&mut self, builder: &DropdownBuilder<u32>, asset_manager: &mut AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        let (id, update_target_collections) = self.widget_registry.create_dropdown(builder, &mut self.element_registry, &mut self.layout_registry, asset_manager)?;
        for update_target_collection in update_target_collections {
            self.handle_ui_update_targets_collection(update_target_collection);
        }
        Ok(id)
    }
    pub fn dropdown_update_result(&self, dropdown_id: &ResourceId<UiWidgetId>) -> Option<u32> {
        self.widget_registry.dropdown_update_result(dropdown_id)
    }

    // icon specific functions
    pub fn create_icon(&mut self, builder: &IconBuilder, asset_manager: &mut AssetManager) -> Result<ResourceId<UiWidgetId>, String> {
        self.widget_registry.create_icon(builder, &mut self.element_registry, asset_manager)
    }
    pub fn set_icon_padding(&mut self, padding: f32, icon_id: &ResourceId<UiWidgetId>) -> Result<(), String> {
        match self.widget_registry.get_widget_by_id(&icon_id) {
            Some(ui_widget) => self.element_registry.set_rectangle_texture_padding(
                &ui_widget.get_main_element_id(&self.widget_registry), 
                padding,
            ),
            None => Err(format!("Icon with id {:?} not found", icon_id)),
        }
    }

    // layout specific functions
    pub fn create_layout(&mut self, builder: &mut impl LayoutBuilder, asset_manager: &mut AssetManager) -> Result<ResourceId<UiLayoutId>, String> {
        let (id, update_targets) = self.layout_registry.create_layout(builder, &mut self.element_registry, &mut self.widget_registry, asset_manager)?;
        self.handle_ui_update_targets_collection(update_targets);
        Ok(id)
    }
    pub fn add_widget_to_layout(&mut self, widget_id: &ResourceId<UiWidgetId>, layout_id: &ResourceId<UiLayoutId>) -> Result<(), String> {
        self.layout_registry.add_widget_to_layout(widget_id, layout_id, &mut self.element_registry, &mut self.widget_registry)
    }

    pub fn set_layout_z_index(&mut self, layout_id: &ResourceId<UiLayoutId>, z_index: f32) {
        self.handle_ui_update_targets_z_index(UiUpdateTargets::from_layout_id(layout_id.clone(), z_index));
    }
    pub fn set_layout_width(&mut self, layout_id: &ResourceId<UiLayoutId>, width: f32) {
        self.handle_ui_update_targets_width(UiUpdateTargets::from_layout_id(layout_id.clone(), width));
    }
    pub fn set_layout_visibility(&mut self, layout_id: &ResourceId<UiLayoutId>, visible: bool) {
        self.handle_ui_update_targets_visibility(UiUpdateTargets::from_layout_id(layout_id.clone(), visible));
    }

    pub fn scroll_speed(&self) -> f32 {
        self.scroll_speed
    }

    fn handle_ui_update_targets_collection(&mut self, targets_collection: UpdateTargetCollection) {
        self.handle_ui_update_targets_draw_bounds(targets_collection.draw_bounds);
        self.handle_ui_update_targets_position(targets_collection.positions);
        self.handle_ui_update_targets_z_index(targets_collection.z_index);
        self.handle_ui_update_targets_visibility(targets_collection.visibility);
        self.handle_ui_update_targets_width(targets_collection.width);
        self.handle_ui_update_targets_height(targets_collection.height);
    }


    // UiUpdateTarget handlers
    fn handle_ui_update_targets_z_index(&mut self, targets: UiUpdateTargets<f32>) {
        for target in targets.layouts {
            let new_targets = self.layout_registry.set_layout_z_index(&target.layout_id, target.data, &mut self.element_registry).unwrap();
            self.handle_ui_update_targets_z_index(new_targets);
        }
        for target in targets.widgets {
            let new_targets = self.widget_registry.set_widget_z_index(&target.widget_id, target.data, &mut self.element_registry);
            self.handle_ui_update_targets_z_index(new_targets);
        }
    }
    fn handle_ui_update_targets_position(&mut self, targets: UiUpdateTargets<Position>) {
        for _target in targets.layouts {
            log::engine_warn("TODO [UI]: implement layout position setter function");
        }
        for target in targets.widgets {
            let new_targets = self.widget_registry.set_widget_position(&target.widget_id, target.data, &mut self.element_registry);
            self.handle_ui_update_targets_position(new_targets);
        }
    }
    fn handle_ui_update_targets_draw_bounds(&mut self, targets: UiUpdateTargets<Bounds2d>) {
        for _target in targets.layouts {
            log::engine_warn("TODO [UI]: implement layout draw bounds setter function");
        }
        for target in targets.widgets {
            let new_targets = self.widget_registry.set_widget_draw_bounds(&target.widget_id, target.data, &mut self.element_registry);
            self.handle_ui_update_targets_draw_bounds(new_targets);
        }
    }
    fn handle_ui_update_targets_width(&mut self, targets: UiUpdateTargets<f32>) {
        for target in targets.layouts {
            let targets_collection = self.layout_registry.set_layout_width(&target.layout_id, target.data, &mut self.element_registry).unwrap();
            self.handle_ui_update_targets_collection(targets_collection);
        }
        for target in targets.widgets {
            let new_targets = self.widget_registry.set_widget_width(&target.widget_id, target.data, &mut self.element_registry);
            self.handle_ui_update_targets_width(new_targets);
        }
    }
    fn handle_ui_update_targets_height(&mut self, targets: UiUpdateTargets<f32>) {
        for _target in targets.layouts {
            log::engine_warn("TODO [UI]: implement layout height setter function");
        }
        for target in targets.widgets {
            let new_targets = self.widget_registry.set_widget_height(&target.widget_id, target.data, &mut self.element_registry);
            self.handle_ui_update_targets_height(new_targets);
        }
    }
    fn handle_ui_update_targets_size(&mut self, targets: UiUpdateTargets<Vec2>) {
        for _target in targets.layouts {
            log::engine_warn("TODO [UI]: implement layout size setter function");
        }
        for target in targets.widgets {
            let new_targets = self.widget_registry.set_widget_size(&target.widget_id, target.data, &mut self.element_registry);
            self.handle_ui_update_targets_size(new_targets);
        }
    }
    fn handle_ui_update_targets_visibility(&mut self, targets: UiUpdateTargets<bool>) {
        for target in targets.layouts {
            let new_targets = self.layout_registry.set_layout_visibility(&target.layout_id, target.data, &mut self.element_registry).unwrap();
            self.handle_ui_update_targets_visibility(new_targets);
        }
        for target in targets.widgets {
            let new_targets = self.widget_registry.set_widget_visibility(&target.widget_id, target.data, &mut self.element_registry);
            self.handle_ui_update_targets_visibility(new_targets);
        }
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

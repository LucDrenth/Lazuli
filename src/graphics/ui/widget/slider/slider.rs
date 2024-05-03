use glam::Vec2;

use crate::{asset_manager::AssetManager, graphics::{font::PlainBitmapBuilder, ui::{self, bounds_2d::Bounds2d, element::{ui_element::UiElement, AnchorPoint}, interface::{self, is_valid_z_index}, widget::UiWidget, ElementRegistry, Position, TextBuilder, UiElementId, UiLayoutId, UiUpdateTargets, UiWidgetId, UpdateTargetCollection}, Color}, input::Input, log, ResourceId};

#[derive(Clone, Copy, Debug)]
pub enum SliderProgressBarAlignment {
    /// Default. Progress bar goes in to the slide direction (follows the mouse).
    Natural,
    Center,
}

#[derive(Clone, Copy, Debug)]
pub enum SliderDirection {
    /// All the way to the left is the minimum value. All the way to the right is the maximum value
    LeftToRight,
    /// All the way to the right is the minimum value. All the way to the left is the maximum value
    RightToLeft,
}

pub struct Slider {
    text_element_id: ResourceId<UiElementId>,
    background_element_id: ResourceId<UiElementId>,
    progress_element_id: ResourceId<UiElementId>,
    progress_bar_alignment: SliderProgressBarAlignment,
    value: f32,
    minimum_value: f32,
    maximum_value: f32,
    decimals: usize,
    scale: Vec2,
    z_index: f32,
    direction: SliderDirection,
    debug: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SliderUpdateResult {
    pub change_amount: f32,
    pub new_value: f32,
    pub did_start_drag: bool,
}

impl UiWidget for Slider {
    fn get_direct_element_ids(&self) -> Vec<ResourceId<UiElementId>> {
        vec![
            self.background_element_id,
            self.text_element_id,
            self.progress_element_id,
        ]
    }
    fn get_direct_layout_ids(&self) -> Vec<ResourceId<UiLayoutId>> {
        vec![]
    }
    fn get_direct_widget_ids(&self) -> Vec<ResourceId<UiWidgetId>> {
        vec![]
    }

    /// Background is the main element. It defines the position and size of the slider
    fn get_main_element_id(&self) -> ResourceId<UiElementId> {
        self.background_element_id.clone()
    }

    fn z_index(&self) -> f32 {
        self.z_index
    }

    fn set_z_index(&mut self, z_index: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        self.z_index = z_index;
        _ = element_registry.set_element_z_index(&self.background_element_id, z_index);
        _ = element_registry.set_element_z_index(&self.text_element_id, z_index + 0.02);
        _ = element_registry.set_element_z_index(&self.progress_element_id, z_index + 0.01);

        UiUpdateTargets::default()
    }

    fn set_position(&self, position: Position, element_registry: &mut ElementRegistry) -> UpdateTargetCollection {
        _ = element_registry.set_element_position(&self.background_element_id, position);
        _ = element_registry.set_element_position(&self.text_element_id, Position::ElementAnchor(AnchorPoint::Center, self.background_element_id));
        _ = element_registry.set_element_position(&self.progress_element_id, Position::ElementAnchor(
            progress_bar_alignment_to_anchor_point(&self.progress_bar_alignment, &self.direction)
            , self.background_element_id
        ));

        UpdateTargetCollection::default()
    }

    fn set_draw_bounds(&self, draw_bounds: Bounds2d, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Bounds2d> {
        _ = element_registry.set_element_draw_bounds(&self.background_element_id, draw_bounds);
        _ = element_registry.set_element_draw_bounds(&self.text_element_id, draw_bounds);
        _ = element_registry.set_element_draw_bounds(&self.progress_element_id, draw_bounds);
        UiUpdateTargets::default()
    }

    fn set_width(&self, width: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        _ = element_registry.set_rectangle_width(&self.background_element_id, width);
        _ = element_registry.set_rectangle_width(&self.progress_element_id, width);
        UiUpdateTargets::default()
    }
    fn set_height(&self, height: f32, element_registry: &mut ElementRegistry) -> UiUpdateTargets<f32> {
        _ = element_registry.set_rectangle_height(&self.background_element_id, height);
        _ = element_registry.set_rectangle_height(&self.progress_element_id, height);
        UiUpdateTargets::default()
    }
    fn set_size(&self, size: Vec2, element_registry: &mut ElementRegistry) -> UiUpdateTargets<Vec2> {
        _ = element_registry.set_rectangle_size(&self.background_element_id, size);
        _ = element_registry.set_rectangle_size(&self.progress_element_id, size);
        UiUpdateTargets::default()
    }

    fn set_visibility(&mut self, visible: bool, element_registry: &mut ElementRegistry) -> UiUpdateTargets<bool> {
        _ = element_registry.set_element_visibility(&self.background_element_id, visible);
        _ = element_registry.set_element_visibility(&self.progress_element_id, visible);
        UiUpdateTargets::default()
    }
    
    fn is_debug(&self) -> bool {
        self.debug
    }
}

impl Slider {
    /// Returns Some if there is a change by dragging the slider
    pub fn update(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) -> Option<SliderUpdateResult> {
        let mut did_start_drag: bool = false;

        match element_registry.get_ui_element_by_id(&self.background_element_id) {
            Some(background_element) => {
                if !background_element.world_data().event_handlers.mouse_left_drag_handler.is_handling() {
                    // We're not dragging. Now we'll check if there was a click to move the slider.
                    if !background_element.world_data().event_handlers.mouse_left_down_handler.did_handle() {
                        return None;
                    }
                } else {
                    did_start_drag = background_element.world_data().event_handlers.mouse_left_drag_handler.did_drag_start();
                }
            },
            None => {
                log::engine_warn(format!("returning None for slider update because we could not find background element with id {}", self.background_element_id.id()));
                return None;
            },
        }

        let old_value = self.value;
        self.handle_drag(input, element_registry, asset_manager);

        Some(SliderUpdateResult{
            change_amount: self.value - old_value,
            new_value: self.value,
            did_start_drag,
        })
    }

    fn handle_drag(&mut self, input: &Input, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) {
        let element_size = element_registry.get_element_size(&self.background_element_id).unwrap();
        let element_position = element_registry.get_element_screen_position(&self.background_element_id).unwrap();

        // The new slider progress as a value from 0.0 to 1.0
        let normalised_value = match self.direction {
            SliderDirection::LeftToRight => {
                let element_start_x = element_position.x - element_size.x / 2.0;
                (element_registry.map_mouse_position(input).x - element_start_x) / element_size.x
            },
            SliderDirection::RightToLeft => {
                let element_end_x = element_position.x + element_size.x / 2.0;
                (element_end_x - element_registry.map_mouse_position(input).x) / element_size.x
            },
        };

        self.set_normalised_value(normalised_value, element_registry, asset_manager);
    }

    pub fn is_hovered(&self, element_registry: &ElementRegistry) -> bool {
        element_registry.is_element_hovered(&self.background_element_id)
    }

    pub fn value(&self) -> f32 { self.value }

    pub fn set_value(&mut self, value: f32, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) {
        self.value = value.clamp(self.minimum_value, self.maximum_value);
        self.update_progress_element(element_registry);
        self.update_text_element(element_registry, asset_manager)
    }

    pub fn translate_value(&mut self, extra_value: f32, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) {
        self.set_value(self.value + extra_value, element_registry, asset_manager);
    }

    pub fn set_normalised_value(&mut self, normalised_value: f32, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) {
        let value = (self.maximum_value - self.minimum_value) * normalised_value + self.minimum_value;
        self.set_value(value, element_registry, asset_manager);
    }

    fn update_progress_element(&self, element_registry: &mut ElementRegistry) {
        let scale = Vec2::new(self.value / (self.maximum_value - self.minimum_value), 1.0) * self.scale;
        
        match element_registry.set_element_scale(&self.progress_element_id, scale) {
            Ok(_) => (),
            Err(err) => {
                log::engine_err(format!("slider update_progress_element failed: {}", err));
            },
        }
    }

    fn update_text_element(&mut self, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) {
        match element_registry.set_text(&self.text_element_id, &Self::value_string(self.value, self.decimals), asset_manager) {
            Ok(_) => (),
            Err(err) => {
                log::engine_warn(format!("failed to update slider text: {}", err));
            },
        }
    }

    fn value_string(value: f32, decimals: usize) -> String {
        format!("{:.decimals$}", value, decimals = decimals)
    }

    pub fn size(&self, element_registry: &ElementRegistry) -> Vec2 {
        element_registry.get_element_size(&self.background_element_id).unwrap()
    }

    pub fn set_scale(&mut self, scale: Vec2, element_registry: &mut ElementRegistry) -> Result<(), String> {
        self.scale = scale;

        element_registry.set_element_scale(&self.background_element_id, scale)?;
        element_registry.set_element_scale(&self.text_element_id, scale)?;
        self.update_progress_element(element_registry);

        Ok(())
    }
}

pub struct SliderBuilder {
    background_color: Color,
    progress_color: Color,
    progress_bar_alignment: SliderProgressBarAlignment,
    text_color: Color,
    z_index: f32,
    position: Position,
    font_path: Option<String>,
    font_size: f32,
    minimum_value: f32,
    maximum_value: f32,
    initial_value: f32,
    width: f32,
    height: f32,
    decimals: usize,
    scale: Vec2,
    direction: SliderDirection,
    is_visible: bool,
    debug: bool,
}

impl SliderBuilder {
    pub fn new() -> Self {
        Self {
            z_index: 10.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            background_color: interface::default_element_background_color(),
            progress_color: Color::Rgb(31, 90, 147),
            progress_bar_alignment: SliderProgressBarAlignment::Natural,
            text_color: interface::default_text_color(),
            font_path: None,
            font_size: interface::default_font_size(),
            minimum_value: 0.0,
            maximum_value: 1.0,
            initial_value: 0.5,
            width: 100.0,
            height: 25.0,
            decimals: 2,
            scale: Vec2::ONE,
            direction: SliderDirection::LeftToRight,
            is_visible: true,
            debug: false,
        }
    }

    pub fn build(&self, element_registry: &mut ElementRegistry, asset_manager: &mut dyn AssetManager) -> Result<Slider, String> {
        let font_id = match &self.font_path {
            Some(font_path) => asset_manager.load_font(&PlainBitmapBuilder::new()
                .with_font_file_path(font_path.clone())
                .with_font_size(50.0)
                , None)?,
            None => interface::default_font(asset_manager)?,
        };

        let background = ui::shapes::RectangleBuilder::new()
            .with_width(self.width)
            .with_height(self.height)
            .with_z_index(self.z_index)
            .with_position(self.position)
            .with_color(self.background_color.clone())
            .with_scale(self.scale)
            .with_visibility(self.is_visible)
            .build(asset_manager, element_registry)?;
        let background_element_id = element_registry.add_rectangle(background);

        let mut progress_rectangle = ui::shapes::RectangleBuilder::new()
            .with_width(self.width)
            .with_height(self.height)
            .with_z_index(self.z_index + 0.01)
            .with_color(self.progress_color.clone())
            .with_position(Position::ElementAnchor(
                progress_bar_alignment_to_anchor_point(&self.progress_bar_alignment, &self.direction), 
                background_element_id
            ))
            .with_scale(Vec2::new(self.initial_value / (self.maximum_value - self.minimum_value), 1.0) * self.scale)
            .with_visibility(self.is_visible)
            .build(asset_manager, element_registry)?;
        progress_rectangle.mut_world_data().event_handlers.set_handle(false);
        let progress_element_id = element_registry.add_rectangle(progress_rectangle);

        let text = TextBuilder::new()
            .with_color(self.text_color.clone())
            .with_z_index(self.z_index + 0.02)
            .with_scale(self.scale)
            .with_position(Position::ElementAnchor(AnchorPoint::Center, background_element_id))
            .with_font_size(self.font_size)
            .with_visibility(self.is_visible)
            .build(Slider::value_string(self.initial_value, self.decimals), &font_id, asset_manager, element_registry)?;
        let text_element_id = element_registry.add_text(text);

        Ok(Slider {
            text_element_id,
            background_element_id,
            progress_element_id,
            progress_bar_alignment: self.progress_bar_alignment,
            value: self.initial_value,
            minimum_value: self.minimum_value,
            maximum_value: self.maximum_value,
            decimals: self.decimals,
            scale: self.scale,
            z_index: self.z_index,
            direction: self.direction,
            debug: self.debug,
        })
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        // Add 1 because the given z_index is for the lowest element, and higher elements may go up to 1 higher
        if is_valid_z_index(z_index + 1.0) {
            self.z_index = z_index;
        } else {
            log::engine_warn(format!("did not set SliderBuilder z_index {} because it's not a valid z-index", z_index));
        }

        self
    }

    pub fn with_text_color(mut self, text_color: Color) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn with_progress_color(mut self, progress_color: Color) -> Self {
        self.progress_color = progress_color;
        self
    }

    pub fn with_progress_bar_alignment(mut self, progress_bar_alignment: SliderProgressBarAlignment) -> Self {
        self.progress_bar_alignment = progress_bar_alignment;
        self
    }

    pub fn with_font_path(mut self, font_path: String) -> Self {
        self.font_path = Some(font_path);
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn with_minimum_value(mut self, minimum_value: f32) -> Self {
        self.minimum_value = minimum_value;
        self
    }

    pub fn with_maximum_value(mut self, maximum_value: f32) -> Self {
        self.maximum_value = maximum_value;
        self
    }

    pub fn with_initial_value(mut self, initial_value: f32) -> Self {
        self.initial_value = initial_value;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn with_decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_direction(mut self, direction: SliderDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.is_visible = visible;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

fn progress_bar_alignment_to_anchor_point(progress_bar_alignment: &SliderProgressBarAlignment, direction: &SliderDirection) -> AnchorPoint {
    match progress_bar_alignment {
        SliderProgressBarAlignment::Natural => {
            match direction {
                SliderDirection::LeftToRight => AnchorPoint::LeftInside(0.0),
                SliderDirection::RightToLeft => AnchorPoint::RightInside(0.0),
            }
        },
        SliderProgressBarAlignment::Center => AnchorPoint::Center,
    }
}

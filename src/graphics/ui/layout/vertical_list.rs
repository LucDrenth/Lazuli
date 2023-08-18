use crate::{graphics::{ui::{ElementRegistry, Position, AnchorPoint, shapes::RectangleBuilder, interface::WidgetRegistry, Interface}, Color}, asset_manager::AssetManager, log};

use super::Layout;

/// TODO separate margins instead of using gap_size as margins

pub struct VerticalList {
    widget_ids: Vec<u32>,
    background_element_id: u32,
    gap_size: f32, // the amount of space between elements
    position: Position,
    max_height: f32,
    current_scroll: f32,
}

impl Layout for VerticalList {}

impl VerticalList {
    pub fn add_widget(&mut self, widget_id: u32, widget_registry: &WidgetRegistry, element_registry: &mut ElementRegistry) {
        // calculate position of newly added widget
        let anchor_id;
        let anchor_point;

        if self.widget_ids.is_empty() {
            anchor_id = self.background_element_id;
            anchor_point = AnchorPoint::TopInside(self.gap_size);
        } else {
            anchor_id = self.widget_ids.last().unwrap().clone();
            anchor_point = AnchorPoint::BottomOutside(self.gap_size);
        }

        // add our new widget to the list if widgets
        self.widget_ids.push(widget_id);

        // TODO resize background element
        let widget_height = widget_registry.get_widget_size(widget_id, element_registry).unwrap().y;

        // reposition background
        self.position = self.position.add_offset(
            0.0, 
            -(widget_height - self.gap_size)
        );
        _ = element_registry.set_element_position(self.background_element_id, self.position);

        // set position of newly added widget
        widget_registry.set_widget_position(widget_id, Position::ElementAnchor(anchor_point, anchor_id), element_registry);
    }

    pub fn update() {
        // TODO update scroll
    }
}

pub struct VerticalListBuilder {
    widget_ids: Vec<u32>, // TODO unique list of ids so we do not have any duplicates
    gap_size: f32, // the amount of space between elements
    background_color: Color,
    max_height: f32,
    position: Position,
}

impl VerticalListBuilder {
    pub fn new() -> Self {
        Self {
            widget_ids: vec![],
            gap_size: 10.0,
            background_color: Color::rgba_black(),
            max_height: 300.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
        }
    }

    pub fn build(self, interface: &mut Interface, asset_manager: &mut AssetManager) -> Result<VerticalList, String> {
        let background_width = self.calculate_background_width(interface);
        let background_height = self.calculate_background_height(interface);

        let background_element_id = interface.mut_element_registry().create_rectangle(RectangleBuilder::new()
            .with_color(self.background_color)
            .with_position(self.position)
            .with_width(background_width)
            .with_height(background_height)
        , asset_manager)?;

        let list = VerticalList { 
            widget_ids: self.widget_ids, 
            background_element_id, 
            gap_size: self.gap_size, 
            position: self.position, 
            max_height: self.max_height,
            current_scroll: 0.0,
        };

        if list.widget_ids.is_empty() {
            return Ok(list);
        }

        // position widgets
        interface.set_widget_position(
            list.widget_ids[0], 
            Position::ElementAnchor(AnchorPoint::TopInside(self.gap_size), list.background_element_id), 
        );

        for i in 1..list.widget_ids.len() {
            let anchor_element = interface.get_widget_anchor_element_id(list.widget_ids[i - 1]).unwrap();
            
            log::engine_info(format!("list widget {} getting anchor widget: {}", list.widget_ids[i], list.widget_ids[i - 1]));
            interface.set_widget_position(
                list.widget_ids[i], 
                Position::ElementAnchor(AnchorPoint::BottomOutside(self.gap_size), anchor_element), 
            );
        }

        Ok(list)
    }

    fn calculate_background_height(&self, interface: &Interface) -> f32 {
        let mut background_height = self.gap_size;

        for widget_id in self.widget_ids.iter() {
            background_height += interface.get_widget_size(*widget_id).unwrap().y + self.gap_size;
        }

        background_height
    }

    /// Returns the width of widest widget + two times the gap size as padding, or only the gap size if there are no elements
    fn calculate_background_width(&self, interface: &Interface) -> f32 {
        if self.widget_ids.is_empty() {
            return self.gap_size;
        }

        let mut widest: f32 = 0.0;

        for widget_id in self.widget_ids.iter() {
            widest = widest.max(interface.get_widget_size(*widget_id).unwrap().x)
        }

        widest + self.gap_size * 2.0
    }


    /*******************************
     * Setter functions start here *
     *******************************/

    pub fn add_widget(mut self, widget_id: u32) -> Self {
        self.widget_ids.push(widget_id);
        self
    }

    pub fn add_widgets(mut self, widget_ids: &mut Vec<u32>) -> Self {
        self.widget_ids.append(widget_ids);
        self
    }

    pub fn with_gap_size(mut self, gap_size: f32) -> Self {
        self.gap_size = gap_size;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }
}

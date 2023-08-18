use crate::{graphics::{ui::{Position, AnchorPoint, shapes::RectangleBuilder, Interface, padding::Padding}, Color}, asset_manager::AssetManager, log};

use super::Layout;

pub struct VerticalList {
    widget_ids: Vec<u32>, // List of unique ids. We do not use a HashSet because the order matters.
    background_element_id: u32,
    gap_size: f32, // the amount of space between elements
    position: Position,
    max_height: f32,
    current_scroll: f32,
    padding: Padding,
}

impl Layout for VerticalList {}

impl VerticalList {
    pub fn add_widget(&mut self, widget_id: u32, interface: &mut Interface) {
        // calculate position of newly added widget
        let anchor_id;
        let anchor_point;

        if self.widget_ids.is_empty() {
            anchor_id = self.background_element_id;
            anchor_point = AnchorPoint::TopInside(self.gap_size);
        } else {
            let anchor_widget_id = self.widget_ids.last().unwrap().clone();
            anchor_id = interface.get_widget_anchor_element_id(anchor_widget_id).unwrap();
            anchor_point = AnchorPoint::BottomOutside(self.gap_size);
        }

        // add our new widget to the list if widgets
        if self.widget_ids.contains(&widget_id) {
            log::engine_warn(format!("VerticalList: widget with id {} was already in the list", widget_id));
            return;
        }
        self.widget_ids.push(widget_id);

        // resize background element
        let new_background_height = calculate_background_height(&self.widget_ids, self.gap_size, &self.padding, interface);
        _ = interface.mut_element_registry().set_rectangle_height(
            self.background_element_id, 
            new_background_height
        );

        // set position of newly added widget
        interface.set_widget_position(widget_id, Position::ElementAnchor(anchor_point, anchor_id));
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
    padding: Padding,
}

impl VerticalListBuilder {
    pub fn new() -> Self {
        let default_gap_size = 10.0;

        Self {
            widget_ids: vec![],
            gap_size: default_gap_size,
            background_color: Color::rgba_black(),
            max_height: 300.0,
            position: Position::ScreenAnchor(AnchorPoint::Center),
            padding: Padding::Universal(default_gap_size),
        }
    }

    pub fn build(self, interface: &mut Interface, asset_manager: &mut AssetManager) -> Result<VerticalList, String> {
        let background_width = calculate_background_width(&self.widget_ids, &self.padding, interface);
        let background_height = calculate_background_height(&self.widget_ids, self.gap_size, &self.padding, interface);

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
            padding: self.padding,
        };

        if list.widget_ids.is_empty() {
            return Ok(list);
        }

        // position widgets
        interface.set_widget_position(
            list.widget_ids[0], 
            Position::ElementAnchor(AnchorPoint::TopInside(list.padding.top()), list.background_element_id), 
        );

        for i in 1..list.widget_ids.len() {
            let anchor_element = interface.get_widget_anchor_element_id(list.widget_ids[i - 1]).unwrap();
            
            interface.set_widget_position(
                list.widget_ids[i], 
                Position::ElementAnchor(AnchorPoint::BottomOutside(self.gap_size), anchor_element), 
            );
        }

        Ok(list)
    }


    /*******************************
     * Setter functions start here *
     *******************************/

    pub fn add_widget(mut self, widget_id: u32) -> Self {
        self.try_add_widget(widget_id);
        self
    }

    pub fn add_widgets(mut self, widget_ids: &Vec<u32>) -> Self {
        // We do not add them all at once because we only want unique values in the list
        for widget_id in widget_ids.iter() {
            self.try_add_widget(*widget_id);
        }

        self
    }

    fn try_add_widget(&mut self, widget_id: u32) {
        if self.widget_ids.contains(&widget_id) {
            log::engine_warn(format!("VerticalListBuilder: widget with id {} was already in the list", widget_id));
            return;
        }

        self.widget_ids.push(widget_id);
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

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
}

fn calculate_background_height(widget_ids: &Vec<u32>, gap_size: f32, padding: &Padding, interface: &Interface) -> f32 {
    let mut background_height = padding.top();

    for widget_id in widget_ids.iter() {
        background_height += interface.get_widget_size(*widget_id).unwrap().y + gap_size;
    }

    background_height - gap_size + padding.bottom()
}

/// Returns the width of widest widget + two times the gap size as padding, or only the gap size if there are no elements
fn calculate_background_width(widget_ids: &Vec<u32>, padding: &Padding, interface: &Interface) -> f32 {
    if widget_ids.is_empty() {
        return padding.horizontal();
    }

    let mut widest: f32 = 0.0;

    for widget_id in widget_ids.iter() {
        widest = widest.max(interface.get_widget_size(*widget_id).unwrap().x)
    }

    widest + padding.horizontal()
}

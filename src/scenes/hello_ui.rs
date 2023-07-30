use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{self, shapes::RectangleBuilder, widget::{Button, ButtonBuilder, Slider, SliderBuilder}, Position, AnchorPoint, TextBuilder}}, event::EventSystem, input::{Input, Key}, asset_registry::AssetRegistry, log};

pub struct HelloUi {
    interface: ui::Interface,
    slider_1: Slider,
    slider_2: Slider,
    reset_button: Button,
    rectangle_id: u32,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system, window_size);

        let rectangle_id = interface.add_rectangle(RectangleBuilder::new(), asset_registry)?;
        

        let slider_1 = Slider::new(SliderBuilder::new(&interface)
            .with_z_index(500.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(20.0, 100.00)))
            .with_initial_value(1.0)
        , &mut interface, asset_registry)?;

        interface.add_text("Rectangle width".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), slider_1.anchor_element_id()))
        , asset_registry)?;


        let slider_2 = Slider::new(SliderBuilder::new(&interface)
            .with_z_index(500.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(20.0, 150.00)))
            .with_initial_value(1.0)
        , &mut interface, asset_registry)?;

        interface.add_text("Rectangle height".to_string(), None, TextBuilder::new()
        .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), slider_2.anchor_element_id()))
        , asset_registry)?;


        
        let reset_button = Button::new("Reset".to_string(), ButtonBuilder::new(&interface)
            .with_position(Position::ScreenAnchor(AnchorPoint::LeftInside(20.0)))
        , &mut interface, asset_registry)?;

        Ok(Self { 
            interface,
            slider_1,
            slider_2,
            rectangle_id,
            reset_button,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_registry: &mut AssetRegistry) {
        self.interface.update(asset_registry, input);

        // TODO update in order of z index
        let result = self.slider_1.update(input, &mut self.interface, asset_registry);
        if result.change.is_some() {
            self.interface.set_element_scale(self.rectangle_id, Vec2 { 
                x: result.value, 
                y: self.interface.get_element_scale(self.rectangle_id).unwrap().y,
            }).unwrap();
        }
        if result.did_start_drag {
            log::engine_info("start drag".to_string());
        }

        let result = self.slider_2.update(input, &mut self.interface, asset_registry);
        if result.change.is_some() {
            self.interface.set_element_scale(self.rectangle_id, Vec2 { 
                x: self.interface.get_element_scale(self.rectangle_id).unwrap().x,
                y: result.value, 
            }).unwrap();
        }

        if self.reset_button.is_clicked(input, &self.interface) {
            self.slider_1.set_value(1.0, &mut self.interface, asset_registry);
            self.slider_2.set_value(1.0, &mut self.interface, asset_registry);
            self.interface.set_element_scale(self.rectangle_id, Vec2::ONE).expect("");
        }

        if input.is_key_down(Key::ArrowDown) {
            _ = self.slider_2.set_scale(Vec2::new(0.7, 0.35), &mut self.interface);
            _ = self.reset_button.set_scale(Vec2::new(3.0, 3.0), &mut self.interface);
        }
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

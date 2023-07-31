use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{shapes::RectangleBuilder, widget::{Button, ButtonBuilder, Slider, SliderBuilder}, Position, AnchorPoint, TextBuilder, Interface}}, event::EventSystem, input::{Input, Key}, asset_registry::AssetRegistry, log};

pub struct HelloUi {
    interface: Interface,
    slider_1: Slider,
    slider_2: Slider,
    reset_button: Button,
    rectangle_id: u32,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface = Interface::new(event_system, window_size);

        let rectangle_id = interface.mut_element_registry().add_rectangle(RectangleBuilder::new(), asset_registry)?;
        
        let slider_1 = Slider::new(SliderBuilder::new(&interface.mut_element_registry())
            .with_z_index(500.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(20.0, 100.00)))
            .with_initial_value(1.0)
        , interface.mut_element_registry(), asset_registry)?;

        interface.mut_element_registry().add_text("Rectangle width".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), slider_1.anchor_element_id()))
        , asset_registry)?;


        let slider_2 = Slider::new(SliderBuilder::new(interface.mut_element_registry())
            .with_z_index(500.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(20.0, 150.00)))
            .with_initial_value(1.0)
        , interface.mut_element_registry(), asset_registry)?;

        interface.mut_element_registry().add_text("Rectangle height".to_string(), None, TextBuilder::new()
        .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), slider_2.anchor_element_id()))
        , asset_registry)?;


        
        let reset_button = Button::new("Reset".to_string(), ButtonBuilder::new(&interface.mut_element_registry())
            .with_position(Position::ScreenAnchor(AnchorPoint::LeftInside(20.0)))
        , interface.mut_element_registry(), asset_registry)?;

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

        // TODO update in order of z index so that if they are (partly) at the same position, we always drag the upper one
        self.slider_1.update(input, self.interface.mut_element_registry(), asset_registry).map(|result| {
            let y = self.interface.mut_element_registry().get_element_scale(self.rectangle_id).unwrap().y;
            self.interface.mut_element_registry().set_element_scale(self.rectangle_id, Vec2 { 
                x: result.new_value, 
                y: y,
            }).unwrap();

            if result.did_start_drag {
                log::engine_info("start drag".to_string());
            }
        });

        self.slider_2.update(input, self.interface.mut_element_registry(), asset_registry).map(|result| {
            let x = self.interface.mut_element_registry().get_element_scale(self.rectangle_id).unwrap().x;
            self.interface.mut_element_registry().set_element_scale(self.rectangle_id, Vec2 { 
                x: x,
                y: result.new_value, 
            }).unwrap();
        });

        if self.reset_button.is_clicked(input, self.interface.mut_element_registry()) {
            self.slider_1.set_value(1.0, self.interface.mut_element_registry(), asset_registry);
            self.slider_2.set_value(1.0, self.interface.mut_element_registry(), asset_registry);
            self.interface.mut_element_registry().set_element_scale(self.rectangle_id, Vec2::ONE).expect("");
        }

        if input.is_key_down(Key::ArrowDown) {
            _ = self.slider_2.set_scale(Vec2::new(0.7, 0.35), self.interface.mut_element_registry());
            _ = self.reset_button.set_scale(Vec2::new(3.0, 3.0), self.interface.mut_element_registry());
        }
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

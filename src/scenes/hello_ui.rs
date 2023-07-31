use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{shapes::RectangleBuilder, widget::{Button, ButtonBuilder, SliderBuilder}, Position, AnchorPoint, TextBuilder, Interface}}, event::EventSystem, input::Input, asset_registry::AssetRegistry, log};

pub struct HelloUi {
    interface: Interface,
    width_slider_id: u32,
    height_slider_id: u32,
    reset_button: Button,
    rectangle_id: u32,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface = Interface::new(event_system, window_size);

        let rectangle_id = interface.mut_element_registry().add_rectangle(RectangleBuilder::new(), asset_registry)?;
        
        let width_slider_id = interface.add_slider(SliderBuilder::new()
            .with_z_index(400.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(10.0, 120.00)))
            .with_initial_value(1.0)
            .with_font_size(20.0)
        , asset_registry)?;

        let anchor = interface.slider_anchor_element_id(width_slider_id).unwrap();
        interface.mut_element_registry().add_text("Rectangle width".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
            .with_z_index(400.0)
        , asset_registry)?;


        let height_slider_id = interface.add_slider(SliderBuilder::new()
            .with_z_index(500.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(10.0, 150.00)))
            .with_initial_value(1.0)
        , asset_registry)?;

        let anchor = interface.slider_anchor_element_id(height_slider_id).unwrap();
        interface.mut_element_registry().add_text("Rectangle height".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
            .with_z_index(500.0)
        , asset_registry)?;


        let reset_button = Button::new("Reset".to_string(), ButtonBuilder::new()
            .with_position(Position::ScreenAnchor(AnchorPoint::LeftInside(20.0)))
        , interface.mut_element_registry(), asset_registry)?;

        Ok(Self { 
            interface,
            width_slider_id,
            height_slider_id,
            rectangle_id,
            reset_button,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_registry: &mut AssetRegistry) {
        self.interface.update(asset_registry, input);

        self.interface.slider_update_result(self.width_slider_id).map(|result|{
            let y = self.interface.mut_element_registry().get_element_scale(self.rectangle_id).unwrap().y;
            self.interface.mut_element_registry().set_element_scale(self.rectangle_id, Vec2 { 
                x: result.new_value, 
                y,
            }).unwrap();

            if result.did_start_drag {
                log::engine_info("start drag".to_string());
            }
        });

        self.interface.slider_update_result(self.height_slider_id).map(|result|{
            let x = self.interface.mut_element_registry().get_element_scale(self.rectangle_id).unwrap().x;
            self.interface.mut_element_registry().set_element_scale(self.rectangle_id, Vec2 { 
                x,
                y: result.new_value, 
            }).unwrap();
        });

        if self.reset_button.is_clicked(input, self.interface.mut_element_registry()) {
            self.interface.set_slider_value(1.0, self.width_slider_id, asset_registry);
            self.interface.set_slider_value(1.0, self.height_slider_id, asset_registry);
            self.interface.mut_element_registry().set_element_scale(self.rectangle_id, Vec2::ONE).expect("");
        }
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

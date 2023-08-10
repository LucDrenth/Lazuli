use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{shapes::RectangleBuilder, widget::{ButtonBuilder, SliderBuilder, Dropdown, DropdownBuilder, DropdownOption}, Position, AnchorPoint, TextBuilder, Interface}, Color}, event::EventSystem, input::{Input, Key, InputAction}, asset_manager::AssetManager, log};

pub struct HelloUi {
    interface: Interface,
    width_slider_id: u32,
    height_slider_id: u32,
    reset_button_id: u32,
    rectangle_id: u32,
    dropdown: Dropdown,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_manager: &mut AssetManager) -> Result<Self, String> 
    {
        let mut interface = Interface::new(event_system, window_size);

        let rectangle_id = interface.mut_element_registry().create_rectangle(RectangleBuilder::new(), asset_manager)?;
        let _ = interface.mut_element_registry().create_rectangle(RectangleBuilder::new()
            .with_color(Color::Rgb(100, 5, 255))
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(5.0), rectangle_id))
        , asset_manager)?;
        
        let width_slider_id = interface.add_slider(SliderBuilder::new()
            .with_z_index(400.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(10.0, 120.00)))
            .with_initial_value(1.0)
        , asset_manager)?;

        let anchor = interface.get_widget_anchor_element_id(width_slider_id).unwrap();
        interface.mut_element_registry().create_text("Rectangle width".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
            .with_z_index(400.0)
        , asset_manager)?;


        let height_slider_id = interface.add_slider(SliderBuilder::new()
            .with_z_index(500.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(10.0, 150.00)))
            .with_initial_value(1.0)
        , asset_manager)?;

        let anchor = interface.get_widget_anchor_element_id(height_slider_id).unwrap();
        interface.mut_element_registry().create_text("Rectangle height".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
            .with_z_index(500.0)
        , asset_manager)?;


        let reset_button_id = interface.add_button("Reset".to_string(), ButtonBuilder::new()
            .with_position(Position::ScreenAnchor(AnchorPoint::LeftInside(20.0)))
            .with_mouse_action_to_activate(InputAction::Up)
        , asset_manager)?;

        let dropdown = Dropdown::new(DropdownBuilder::new()
            .with_placeholder_text("--- selected a fruit ---".to_string())
            .with_options(vec![
                DropdownOption{ label: "Banana".to_string(), value: 1 },
                DropdownOption{ label: "Strawberry".to_string(), value: 2 },
                DropdownOption{ label: "Blackberry".to_string(), value: 3 },
                DropdownOption{ label: "Apple".to_string(), value: 4 },
            ])
            .with_position(Position::ScreenAnchor(AnchorPoint::TopLeftInside(10.0, 10.0)))
        , interface.mut_element_registry(), asset_manager)?;

        Ok(Self { 
            interface,
            width_slider_id,
            height_slider_id,
            rectangle_id,
            reset_button_id,
            dropdown,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_manager: &mut AssetManager) {
        self.interface.update(asset_manager, input);

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

        if self.interface.is_button_clicked(self.reset_button_id) {
            self.interface.set_slider_value(1.0, self.width_slider_id, asset_manager);
            self.interface.set_slider_value(1.0, self.height_slider_id, asset_manager);
            self.interface.mut_element_registry().set_element_scale(self.rectangle_id, Vec2::ONE).unwrap();
        }

        if input.is_key_down(Key::ArrowUp) {
            _ = self.interface.show_widget(self.reset_button_id);
        }
        if input.is_key_down(Key::ArrowDown) {
            _ = self.interface.hide_widget(self.reset_button_id);
        }

        match self.dropdown.update(input, self.interface.mut_element_registry(), asset_manager) {
            Some(new_value) => log::info(format!("new dropdown value: {}", new_value)),
            None => (),
        }
    }

    unsafe fn draw(&self, asset_manager: &mut AssetManager) {
        self.interface.draw(asset_manager);
    }
}

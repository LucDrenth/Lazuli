use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{shapes::RectangleBuilder, widget::{ButtonBuilder, SliderBuilder, DropdownBuilder, DropdownOption}, Position, AnchorPoint, TextBuilder, Interface, VerticalListBuilder, Padding, VerticalList, Layout, UiWidgetId, UiElementId}, Color}, event::EventSystem, input::{Input, Key}, asset_manager::AssetManager, log, ResourceId};

pub struct HelloUi {
    interface: Interface,
    width_slider_id: ResourceId<UiWidgetId>,
    height_slider_id: ResourceId<UiWidgetId>,
    reset_button_id: ResourceId<UiWidgetId>,
    rectangle_id: ResourceId<UiElementId>,
    dropdown_id: ResourceId<UiWidgetId>,
    layout: VerticalList,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, window_size: Vec2, pixel_density: f32, asset_manager: &mut AssetManager) -> Result<Self, String> 
    {
        let mut interface = Interface::new(event_system, window_size, pixel_density);

        let rectangle_id = interface.mut_element_registry().create_rectangle(RectangleBuilder::new(), asset_manager)?;
        let _ = interface.mut_element_registry().create_rectangle(RectangleBuilder::new()
            .with_color(Color::Rgb(100, 5, 255))
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(5.0), rectangle_id))
        , asset_manager)?;

        let mut slider_size_builder = SliderBuilder::new()
            .with_initial_value(1.0);
        let width_slider_id = interface.add_slider(&slider_size_builder, asset_manager)?;

        let anchor = interface.get_widget_main_element_id(&width_slider_id).unwrap();
        let _width_slider_label_id = interface.mut_element_registry().create_text("Rectangle width".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
        , asset_manager)?;


        slider_size_builder = slider_size_builder
            .with_z_index(500.0)
            .with_initial_value(1.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(10.0, 10.0)));
        let height_slider_id = interface.add_slider(&slider_size_builder, asset_manager)?;

        let anchor = interface.get_widget_main_element_id(&height_slider_id).unwrap();
        let _height_slider_label_id = interface.mut_element_registry().create_text("Rectangle height".to_string(), None, TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
            .with_z_index(500.0)
        , asset_manager)?;


        let reset_button_id = interface.add_button("Reset", &ButtonBuilder::new()
        , asset_manager)?;

        let dropdown_id = interface.add_dropdown(&DropdownBuilder::new()
            .with_placeholder_text("--- selected a color ---")
            .with_options(vec![
                DropdownOption::new("Red", 1 ),
                DropdownOption::new("Green", 2 ),
                DropdownOption::new("Blue", 3 ),
                DropdownOption::new("Yellow", 4 ),
                DropdownOption::new("Pink", 5 ),
                DropdownOption::new("Cyan", 6 ),
            ])
            .with_position(Position::ScreenAnchor(AnchorPoint::TopLeftInside(10.0, 10.0)))
        , asset_manager)?;

        let layout_button_builder = ButtonBuilder::new();
        let layout_button_1 = interface.add_button("Button 1", &layout_button_builder, asset_manager)?;
        let layout_button_2 = interface.add_button("Button 2", &layout_button_builder, asset_manager)?;
        let layout_button_3 = interface.add_button("Button 3", &layout_button_builder, asset_manager)?;
        let layout_button_4 = interface.add_button("Button 4", &layout_button_builder, asset_manager)?;
        let layout_button_5 = interface.add_button("Button 5", &layout_button_builder, asset_manager)?;

        let layout = VerticalListBuilder::new()
            .with_position(Position::ScreenAnchor(AnchorPoint::LeftInside(10.0)))
            .with_padding(Padding::Universal(10.0))
            .with_max_height(230.0)
            .add_widget(&dropdown_id)
            .add_widget(&reset_button_id)
            .add_widget(&width_slider_id)
            .add_widget(&layout_button_1)
            .add_widget(&layout_button_2)
            .add_widget(&layout_button_3)
            .add_widget(&layout_button_4)
            .add_widget(&layout_button_5)
            .build(&mut interface, asset_manager)?;

        Ok(Self { 
            interface,
            width_slider_id,
            height_slider_id,
            rectangle_id,
            reset_button_id,
            dropdown_id,
            layout,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_manager: &mut AssetManager) {
        self.interface.update(asset_manager, input);
        self.layout.update(&mut self.interface, input);

        self.interface.slider_update_result(&self.width_slider_id).map(|result|{
            let y = self.interface.mut_element_registry().get_element_scale(&self.rectangle_id).unwrap().y;
            self.interface.mut_element_registry().set_element_scale(&self.rectangle_id, Vec2 { 
                x: result.new_value, 
                y,
            }).unwrap();

            if result.did_start_drag {
                log::engine_info("start drag".to_string());
            }
        });

        self.interface.slider_update_result(&self.height_slider_id).map(|result|{
            let x = self.interface.mut_element_registry().get_element_scale(&self.rectangle_id).unwrap().x;
            self.interface.mut_element_registry().set_element_scale(&self.rectangle_id, Vec2 { 
                x,
                y: result.new_value, 
            }).unwrap();
        });

        if self.interface.is_button_clicked(&self.reset_button_id) {
            self.interface.set_slider_value(1.0, &self.width_slider_id, asset_manager);
            self.interface.set_slider_value(1.0, &self.height_slider_id, asset_manager);
            self.interface.mut_element_registry().set_element_scale(&self.rectangle_id, Vec2::ONE).unwrap();
        }

        if input.is_key_down(Key::ArrowUp) {
            _ = self.interface.show_widget(&self.reset_button_id);
        }
        if input.is_key_down(Key::ArrowDown) {
            _ = self.interface.hide_widget(&self.reset_button_id);
        }

        match self.interface.dropdown_update_result(&self.dropdown_id) {
            Some(new_value) => {
                let color = match new_value {
                    1 => Color::hex("#ff0000"),
                    2 => Color::hex("#00ff00"),
                    3 => Color::hex("#0000ff"),
                    4 => Color::hex("#ffff00"),
                    5 => Color::hex("#ff00ff"),
                    6 => Color::hex("#00ffff"),
                    _ => {
                        log::warn(format!("unhandled dropdown value: {}", new_value));
                        Color::hex("#ffffff")
                    },
                };

                _ = self.interface.mut_element_registry().set_element_color(&self.rectangle_id, color.clone());
                _ = self.interface.set_button_text_color(color, &self.reset_button_id);
            },
            None => (),
        }

        if input.is_key_down(Key::Space) {
            self.layout.add_widget(&self.height_slider_id, &mut self.interface);
        }
    }

    unsafe fn draw(&self, asset_manager: &mut AssetManager) {
        self.interface.draw(asset_manager);
    }
}

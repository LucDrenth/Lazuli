use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{shapes::RectangleBuilder, widget::{ButtonBuilder, SliderBuilder, DropdownBuilder, DropdownOption}, Position, AnchorPoint, TextBuilder, Interface, VerticalListBuilder, Padding, UiWidgetId, UiElementId, Width, UiLayoutId}, Color}, event::EventSystem, input::{Input, Key}, asset_manager::AssetManager, log, ResourceId};

pub struct HelloUi {
    width_slider_id: ResourceId<UiWidgetId>,
    height_slider_id: ResourceId<UiWidgetId>,
    reset_button_id: ResourceId<UiWidgetId>,
    rectangle_id: ResourceId<UiElementId>,
    dropdown_id: ResourceId<UiWidgetId>,
    layout_id: ResourceId<UiLayoutId>,
    mouse_pos: ResourceId<UiElementId>,
}

impl Scene for HelloUi {
    fn new(_: &mut EventSystem, _: Vec2, _: f32, asset_manager: &mut AssetManager, interface: &mut Interface) -> Result<Self, String> 
    {
        let rectangle_id = interface.mut_element_registry().create_rectangle(&RectangleBuilder::new(), asset_manager)?;
        let _ = interface.mut_element_registry().create_rectangle(&RectangleBuilder::new()
            .with_color(Color::Rgb(100, 5, 255))
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(5.0), rectangle_id))
        , asset_manager)?;

        
        let width_slider_id = interface.create_slider(&SliderBuilder::new()
            .with_initial_value(1.0)
            .with_debug(true)
        , asset_manager)?;

        let anchor = interface.get_widget_main_element_id(&width_slider_id).unwrap();
        let _width_slider_label_id = interface.mut_element_registry().create_text("Rectangle width".to_string(), None, &TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
        , asset_manager)?;

        let height_slider_id = interface.create_slider(&SliderBuilder::new()
            .with_initial_value(1.0)
            .with_z_index(500.0)
            .with_initial_value(1.0)
            .with_position(Position::ScreenAnchor(AnchorPoint::BottomLeftInside(10.0, 10.0)))
        , asset_manager)?;

        let anchor = interface.get_widget_main_element_id(&height_slider_id).unwrap();
        let _height_slider_label_id = interface.mut_element_registry().create_text("Rectangle height".to_string(), None, &TextBuilder::new()
            .with_position(Position::ElementAnchor(AnchorPoint::RightOutside(10.0), anchor))
            .with_z_index(500.0)
        , asset_manager)?;


        let reset_button_id = interface.create_button("Reset", &ButtonBuilder::new()
        , asset_manager)?;

        let dropdown_id = interface.create_dropdown(&DropdownBuilder::new()
            .with_placeholder_text("--- select a color ---")
            .with_options(vec![
                DropdownOption::new("Red", 1),
                DropdownOption::new("Green", 2),
                DropdownOption::new("Blue", 3),
                DropdownOption::new("Yellow", 4),
                DropdownOption::new("Pink", 5),
                DropdownOption::new("Cyan", 6),
            ])
            .with_position(Position::ScreenAnchor(AnchorPoint::TopLeftInside(10.0, 10.0)))
        , asset_manager)?;

        let layout_button_builder = ButtonBuilder::new();

        let mut vertical_list_builder = VerticalListBuilder::new()
            .with_position(Position::ScreenAnchor(AnchorPoint::LeftInside(10.0)))
            .with_padding(Padding::Universal(10.0))
            .with_max_height(230.0)
            .with_width(Width::Fixed(175.0))
            .add_widget(&dropdown_id)
            .add_widget(&reset_button_id)
            .add_widget(&width_slider_id)
            ;

        for i in 0..=25 {
            let button_id = interface.create_button(format!("Button {i}"), &layout_button_builder, asset_manager)?;
            vertical_list_builder = vertical_list_builder.add_widget(&button_id);
        }
        
        let layout_id = interface.create_layout(&mut vertical_list_builder, asset_manager)?;

        let mouse_pos = interface.mut_element_registry().create_text("", None, &TextBuilder::new()
                .with_position(Position::ScreenAnchor(AnchorPoint::TopRightInside(10., 10.)))
        , asset_manager)?;

        Ok(Self { 
            width_slider_id,
            height_slider_id,
            rectangle_id,
            reset_button_id,
            dropdown_id,
            layout_id,
            mouse_pos,
        })
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_manager: &mut AssetManager, interface: &mut Interface) {
        interface.slider_update_result(&self.width_slider_id).map(|result|{
            let y = interface.mut_element_registry().get_element_scale(&self.rectangle_id).unwrap().y;
            interface.mut_element_registry().set_element_scale(&self.rectangle_id, Vec2 { 
                x: result.new_value, 
                y,
            }).unwrap();

            if result.did_start_drag {
                log::info("start drag".to_string());
            }
        });

        interface.slider_update_result(&self.height_slider_id).map(|result|{
            let x = interface.mut_element_registry().get_element_scale(&self.rectangle_id).unwrap().x;
            interface.mut_element_registry().set_element_scale(&self.rectangle_id, Vec2 { 
                x,
                y: result.new_value, 
            }).unwrap();
        });

        if interface.is_button_clicked(&self.reset_button_id) {
            interface.set_slider_value(1.0, &self.width_slider_id, asset_manager);
            interface.set_slider_value(1.0, &self.height_slider_id, asset_manager);
            interface.mut_element_registry().set_element_scale(&self.rectangle_id, Vec2::ONE).unwrap();
        }

        if input.is_key_down(Key::ArrowUp) {
            _ = interface.set_widget_visibility(&self.reset_button_id, true);
        }
        if input.is_key_down(Key::ArrowDown) {
            _ = interface.set_widget_visibility(&self.reset_button_id, false);
        }

        match interface.dropdown_update_result(&self.dropdown_id) {
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

                _ = interface.mut_element_registry().set_element_color(&self.rectangle_id, color.clone());
                _ = interface.set_button_text_color(color, &self.reset_button_id);
            },
            None => (),
        }

        if input.is_key_down(Key::Space) {
            _ = interface.add_widget_to_layout(&self.height_slider_id, &self.layout_id);
        }

        let mouse_pos = input.get_mouse_position() - interface.size() / 2.0;
        let mouse_pos_text = format!("{}, {}", mouse_pos.x as i32, mouse_pos.y as i32);
        _ = interface.mut_element_registry().set_text(&self.mouse_pos, &mouse_pos_text, asset_manager);
    }

    unsafe fn draw(&self, _: &mut AssetManager) {
    }
}

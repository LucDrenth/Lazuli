use glam::Vec2;
use lazuli::{scenes::*, graphics::{window::WindowBuilder, scene::Scene, ui::{Interface, UiWidgetId, widget::{DropdownBuilder, DropdownOption, SliderBuilder}, AnchorPoint, UiElementId, shapes::RectangleBuilder}, Color}, event::EventSystem, asset_manager::AssetManager, input::Input, ResourceId};

fn main() {
    let window_builder = WindowBuilder::new()
        .with_name("Lazuli app")
    ;

    lazuli::run_scene::<HelloUi>(window_builder);
}

pub struct CustomScene {
    dropdown: ResourceId<UiWidgetId>,
    slider: ResourceId<UiWidgetId>,
}

impl Scene for CustomScene {
    fn new(_: &mut EventSystem, _: Vec2, _: f32, asset_manager: &mut AssetManager, interface: &mut Interface) -> Result<Self, String> where Self: Sized {
        let dropdown = interface.create_dropdown(&DropdownBuilder::new()
            .with_options(vec![
                DropdownOption::new("1", 1),
                DropdownOption::new("2", 2),
                DropdownOption::new("3", 3),
            ])
        , asset_manager)?;

        let slider = interface.create_slider(&SliderBuilder::new()
            .with_position(lazuli::graphics::ui::Position::ScreenAnchor(AnchorPoint::LeftInside(10.0)))
        , asset_manager)?;

        Ok(Self { 
            dropdown,
            slider,
        })
    }

    unsafe fn draw(&self, _: &mut AssetManager) {
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetManager, interface: &mut Interface) {
        interface.dropdown_update_result(&self.dropdown).map(|e|{
            lazuli::log::info(format!("new dropdown value: {}", e));
        });
    }
}

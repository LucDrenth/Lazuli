use glam::Vec2;

use crate::{graphics::{scene::Scene, ui::{self, shapes::{Rectangle, RectangleBuilder}}}, event::EventSystem, input::Input};

pub struct HelloUi {
    interface: ui::Interface,
}

impl Scene for HelloUi {
    fn new(event_system: &mut EventSystem, _window_size: Vec2) -> Result<Self, String> 
    {
        let mut interface: ui::Interface = ui::Interface::new(event_system);

        let rectangle = Box::new(Rectangle::new(RectangleBuilder::new()
            .with_color((255, 25, 162))
        , &mut interface)?);
        interface.add_element(rectangle);

        let result = Self { 
            interface,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _input: &Input) {
        self.interface.update();
    }

    unsafe fn draw(&self) {
        self.interface.draw();
    }
}

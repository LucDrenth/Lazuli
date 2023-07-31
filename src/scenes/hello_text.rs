use glam::Vec2;

use crate::{graphics::{scene::Scene, font::PlainBitmapBuilder, ui::{TextBuilder, Interface}}, event::EventSystem, input::Input, asset_registry::AssetRegistry};

pub struct HelloText {
    interface: Interface,
}

impl Scene for HelloText {
    fn new(event_system: &mut EventSystem, window_size: Vec2, asset_registry: &mut AssetRegistry) -> Result<Self, String> 
    {
        let mut interface = Interface::new(event_system, window_size);


        let plain_font_id = asset_registry.load_font(PlainBitmapBuilder::new()
            .with_font_size(50.0)
        , None)?;
        
        interface.mut_element_registry().add_text("Welcome to Lazuli engine".to_string(), Some(&plain_font_id), TextBuilder::new()
            .with_font_size(25.0)
            .with_color((255, 255, 255))
            .with_letter_spacing(0.05)
        , asset_registry)?;



        /////////////////////////////////////////////////////////////////////////////////
        // The following is an example of SDF text, which is not fully implemented yet //
        /////////////////////////////////////////////////////////////////////////////////

        // let sdf_font_id = asset_registry.load_font(SdfBitmapBuilder::new()
        //     .with_font_size(50.0)
        //     .with_spread(8)
        //     .with_super_sampling_factor(4)
        // , None)?;
        // let sdf_text = Text::new("Welcome to Lazuli engine".to_string(), sdf_font_id, TextBuilder::new()
        //     .with_font_size(25.0)
        //     .with_color((255, 255, 255))
        //     .with_letter_spacing(0.05)
        //     .with_position(ui::Position::FixedBottom(-100.0))
        // , asset_registry, &element_registry.size())?;
        // element_registry.add_element(sdf_text);
        

        let result = Self { 
            interface,
        };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, input: &Input, asset_registry: &mut AssetRegistry) {
        self.interface.update(asset_registry, input);
    }

    unsafe fn draw(&self, asset_registry: &mut AssetRegistry) {
        self.interface.draw(asset_registry);
    }
}

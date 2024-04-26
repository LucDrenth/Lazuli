use glam::Vec2;

use crate::{asset_manager::AssetManager, event::EventSystem, graphics::{font::PlainBitmapBuilder, scene::Scene, ui::{Interface, TextBuilder}, Color}, input::Input};

pub struct HelloText {
}

impl Scene for HelloText {
    fn new(_: &mut EventSystem, _: Vec2, _: f32, asset_manager: &mut dyn AssetManager, interface: &mut Interface) -> Result<Self, String> 
    {
        let plain_font_id = asset_manager.load_font(&PlainBitmapBuilder::new()
            .with_font_size(50.0)
        , None)?;
        
        interface.mut_element_registry().create_text("Welcome to Lazuli engine".to_string(), Some(&plain_font_id), &TextBuilder::new()
            .with_font_size(25.0)
            .with_color(Color::Rgb(255, 255, 255))
            .with_letter_spacing(0.05)
        , asset_manager)?;



        /////////////////////////////////////////////////////////////////////////////////
        // The following is an example of SDF text, which is not fully implemented yet //
        /////////////////////////////////////////////////////////////////////////////////

        // let sdf_font_id = asset_manager.load_font(SdfBitmapBuilder::new()
        //     .with_font_size(50.0)
        //     .with_spread(8)
        //     .with_super_sampling_factor(4)
        // , None)?;
        // let sdf_text = Text::new("Welcome to Lazuli engine".to_string(), sdf_font_id, TextBuilder::new()
        //     .with_font_size(25.0)
        //     .with_color((255, 255, 255))
        //     .with_letter_spacing(0.05)
        //     .with_position(ui::Position::FixedBottom(-100.0))
        // , asset_manager, &element_registry.size())?;
        // element_registry.add_element(sdf_text);
        

        let result = Self { };

        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut dyn AssetManager, _: &mut Interface) {
    }

    unsafe fn draw(&self, _: &mut dyn AssetManager) {
    }
}

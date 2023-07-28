use std::{fs, path::Path};

use glam::Vec2;

use serde::{Deserialize, Serialize};
use crate::{graphics::scene::Scene, event::EventSystem, input::Input, asset_registry::AssetRegistry, log};

pub struct HelloJson {}
impl Scene for HelloJson {
    fn new(_event_system: &mut EventSystem, _window_size: Vec2, _: &mut AssetRegistry) -> Result<Self, String>
    {
        let data = MyStruct {
            id: 15,
            name: "test Luc".to_string(),
        };
        let json = serde_json::to_string(&data).unwrap();
        fs::write(Path::new("./data.json"), json).unwrap();

        let json_to_read = fs::read_to_string(Path::new("./data.json")).unwrap();
        let my_struct: MyStruct = serde_json::from_str(&json_to_read).unwrap();
        log::engine_info(format!("{}: {}", my_struct.id, my_struct.name));

        let result = Self { };
        Ok(result)
    }

    fn update(&mut self, _: &mut EventSystem, _: &Input, _: &mut AssetRegistry) {}
    unsafe fn draw(&self, _: &mut AssetRegistry) {}
}

#[derive(Serialize, Deserialize)]
struct MyStruct {
    pub id: u8,
    pub name: String,
}

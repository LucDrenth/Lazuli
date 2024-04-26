use app::App;
use graphics::{scene::Scene, window::WindowBuilder};

pub mod graphics;
pub mod scenes;
pub mod error;
pub mod event;
pub mod input;
pub mod time;
pub mod math;
pub mod asset_manager;

// TODO only expose the 3 commented out functions from `mod log`
pub mod log;
// pub use log::info;
// pub use log::warn;
// pub use log::err;

pub extern crate glam;
pub extern crate gl;
pub extern crate rand;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate chrono;
pub extern crate image;

mod resource_id;
pub use resource_id::ResourceId;

mod app;

pub fn run_scene<T: Scene + 'static>(window_builder: WindowBuilder) {
    App::new::<T>(window_builder);
}

pub fn hello_triangle() {
    run_scene::<scenes::HelloTriangle>(WindowBuilder::new());
}

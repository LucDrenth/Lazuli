use app::App;
use graphics::scene::Scene;

pub mod graphics;
pub mod scenes;
pub mod error;
pub mod event;
pub mod input;
pub mod time;
pub mod math;
pub mod asset_manager;

mod log;
pub use log::info;
pub use log::warn;
pub use log::err;

pub extern crate glam;
pub extern crate gl;
pub extern crate rand;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate chrono;
pub extern crate image;

mod app;

pub fn run_scene<T: Scene + 'static>() {
    App::new::<T>();
}

pub fn hello_triangle() {
    run_scene::<scenes::HelloTriangle>();
}

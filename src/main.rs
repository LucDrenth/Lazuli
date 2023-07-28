use app::App;
use scenes::HelloUi as InitialScene;

mod graphics;
mod scenes;
mod error;
mod log;
mod event;
mod input;
mod time;
mod math;
mod asset_registry;

mod app;

fn main() {
    App::new::<InitialScene>();
}

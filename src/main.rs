use app::App;
use graphics::scene::Scene;
use scenes::HelloText as InitialScene;

mod graphics;
mod scenes;
mod error;
mod log;
mod event;
mod input;
mod time;
mod math;

mod app;

fn main() {
    let mut app = App::new();
    let window_size = app.window_size();
    let scene = InitialScene::new(&mut app.event_system, window_size).expect("Could not create scene");
    app.run(Box::new(scene)); 
}

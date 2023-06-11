use app::App;
use scenes::EventBusScene as Scene;

mod graphics;
mod scenes;
mod error;
mod log;
mod event;

mod app;

fn main() {
    let mut app = App::new();
    let scene = Scene::new(&mut app.event_system).expect("Could not create scene");
    app.run(Box::new(scene)); 
}

use app::App;
use graphics::scene::Scene;
use scenes::EventBusScene as InitialScene;

mod graphics;
mod scenes;
mod error;
mod log;
mod event;

mod app;

fn main() {
    let mut app = App::new();
    let scene = InitialScene::new(&mut app.event_system).expect("Could not create scene");
    app.run(Box::new(scene)); 
}

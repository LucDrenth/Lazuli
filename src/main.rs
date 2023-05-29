use app::App;
use scenes::MovingTriangle as Scene;

mod graphics;
mod scenes;
mod error;
mod log;

mod app;

fn main() {
    let app = App::new();
    let scene = Scene::new().expect("Could not create scene");
    app.run(Box::new(scene)); 
}

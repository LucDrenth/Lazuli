use app::App;
use scenes::basic_scene::BasicScene;

mod graphics;
mod scenes;
mod error;

mod app;

fn main() {
    let app = App::new();
    let scene = BasicScene::new().expect("Could not create scene");
    app.run(Box::new(scene)); 
}

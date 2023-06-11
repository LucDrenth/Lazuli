use app::App;
use scenes::EventBusScene as Scene;

mod graphics;
mod scenes;
mod error;
mod log;
mod event;

mod app;

fn main() {
    event::do_test();

    
    let app = App::new();
    let scene = Scene::new().expect("Could not create scene");
    app.run(Box::new(scene)); 

}

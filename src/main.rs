use std::env;

use graphics::renderer::Renderer;
use scenes::basic_scene::BasicScene;

mod graphics;
mod error;
pub mod scenes;

fn main() {
    // TODO since backtrace can be slow in production, we need to disable this in release mode
    env::set_var("RUST_BACKTRACE", "1");

    let window = graphics::Window::new(String::from("Lazuli"));
    let scene = BasicScene::new().expect("Could not create scene");
    let renderer = Renderer::new(Box::new(scene)).expect("Could not create renderer");
    window.run(renderer);
}

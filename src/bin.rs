use lazuli::{scenes::*, graphics::window::WindowBuilder};

fn main() {
    let window_builder = WindowBuilder::new()
        .with_name("Lazuli app")
    ;

    lazuli::run_scene::<CoordinateSystem>(window_builder);
}

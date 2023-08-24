mod custom_scene;

fn main() {
    // To run a custom scene (./custom_scene.rs), remove the line below and
    // uncomment the commented out lines
    
    lazuli::hello_triangle();

    // lazuli::run_scene::<custom_scene::CustomScene>(
    //     lazuli::graphics::window::WindowBuilder::new()
    // );
}

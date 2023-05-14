use std::env;

mod graphics;
mod error;

fn main() {
    // TODO since backtrace can be slow in production, we need to disable this in release mode
    env::set_var("RUST_BACKTRACE", "1");

    graphics::window::run(String::from("Lazuli"));
}

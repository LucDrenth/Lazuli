pub trait Scene {
    unsafe fn draw(&self);
    fn update(&mut self);
}

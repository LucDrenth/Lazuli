pub enum Event {
    WindowResize(WindowResizeEvent) // width x height
}

pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

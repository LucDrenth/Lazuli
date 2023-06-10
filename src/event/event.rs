
#[derive(Eq, Hash, PartialEq)]
pub enum Event {
    WindowResize(WindowResizeEvent), 
}

#[derive(Eq, Hash, PartialEq)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

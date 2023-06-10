
#[derive(Eq, Hash, PartialEq)]
pub enum Event {
    WindowResize(WindowResizeEvent) // width x height
}

impl Event {
    pub fn type_to_string(&self) -> &str {
        match self {
            Event::WindowResize(_) => "WindowResize",
            _ => "undefined"
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

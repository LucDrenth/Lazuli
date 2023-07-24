// Window events
#[derive(Clone, Debug)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

// Cursor events
#[derive(Clone)]
pub struct LockCursor {}
#[derive(Clone)]
pub struct UnlockCursor {}
#[derive(Clone)]
pub struct ConfineCursor {}
#[derive(Clone)]
pub struct HideCursor {}
#[derive(Clone)]
pub struct ShowCursor {}
#[derive(Clone)]
pub struct SetCursorPosition {
    pub x: f32,
    pub y: f32,
}

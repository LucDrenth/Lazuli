// Window events
#[derive(Clone, Debug)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct PixelDensityChangeEvent {
    pub pixel_density: f32,
}

// Cursor events
#[derive(Clone, Debug)]
pub struct LockCursor {}
#[derive(Clone, Debug)]
pub struct UnlockCursor {}
#[derive(Clone, Debug)]
pub struct ConfineCursor {}
#[derive(Clone, Debug)]
pub struct HideCursor {}
#[derive(Clone, Debug)]
pub struct ShowCursor {}
#[derive(Clone, Debug)]
pub struct SetCursorPosition {
    pub x: f32,
    pub y: f32,
}

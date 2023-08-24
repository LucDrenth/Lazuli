pub struct WindowBuilder {
    pub name: String,
    pub size: WindowSize,
}

pub enum WindowSize {
    FullScreen, // As big as the monitor, removing OS HUD
    Maximized, // As big as the monitor, keeping the OS HUD
    Pixels(u32, u32)
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self { 
            name: "Lazuli project".to_string(),
            size: WindowSize::Pixels(800, 600),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = WindowSize::Pixels(width, height);
        self
    }

    pub fn with_full_screen(mut self) -> Self {
        self.size = WindowSize::FullScreen;
        self
    }

    pub fn with_maximized(mut self) -> Self {
        self.size = WindowSize::Maximized;
        self
    }
}

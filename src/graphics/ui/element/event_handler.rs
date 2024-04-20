#[derive(Clone, Copy)]
pub struct EventHandler {
    /// If false, the event is never handled
    handle: bool,

    /// When set to true, do not check any further element for this event.
    /// When set to false, keep on checking for elements that handle the event until we come
    /// across one that handles it and has capture set to true.
    capture: bool,

    did_handle: bool,
}

impl EventHandler{
    pub fn new(handle: bool, capture: bool) -> Self {
        Self { 
            handle, 
            capture, 
            did_handle: false 
        }
    }

    pub fn set_handle(&mut self, handle: bool) {
        self.handle = handle
    }
    pub fn set_capture(&mut self, capture: bool) {
        self.capture = capture
    }

    pub fn try_to_handle(&mut self) {
        if self.handle {
            self.did_handle = true;
        }
    }

    pub fn did_handle(&self) -> bool {
        self.did_handle
    }

    pub fn reset(&mut self) {
        self.did_handle = false;
    }

    pub fn did_capture(&self) -> bool {
        self.handle && self.capture
    }
}

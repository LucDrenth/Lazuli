#[derive(Clone, Copy)]
pub struct EventHandler {
    /// If false, the event is never handled
    does_handle: bool,

    /// When set to true, do not check any further element for this event.
    /// When set to false, keep on checking for elements that handle the event until we come
    /// across one that handles it and has capture set to true.
    capture: bool,

    did_handle: bool,
}

impl EventHandler{
    pub fn new(handle: bool, capture: bool) -> Self {
        Self { 
            does_handle: handle, 
            capture, 
            did_handle: false 
        }
    }

    pub fn set_does_handle(&mut self, does_handle: bool) {
        self.does_handle = does_handle
    }
    pub fn set_capture(&mut self, capture: bool) {
        self.capture = capture
    }

    pub fn try_to_handle(&mut self) {
        if self.does_handle {
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
        self.does_handle && self.capture
    }
}

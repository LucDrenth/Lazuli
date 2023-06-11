pub mod event;
pub use event::Event;

pub mod event_bus;
pub use event_bus::add_listener_window_resize;
pub use event_bus::send;

mod event_system;
pub use event_system::do_test;

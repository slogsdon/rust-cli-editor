use super::{input::WindowInputEvent, terminal::get_window_size};

pub struct WindowState {
    pub dimensions: (u16, u16),
    pub event_history: Vec<WindowInputEvent>,
}

impl WindowState {
    pub fn new() -> WindowState {
        WindowState {
            dimensions: get_window_size(),
            event_history: Vec::<WindowInputEvent>::new(),
        }
    }
}

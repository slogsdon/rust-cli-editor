/// Provides options for maintaining application state
use crossterm::event::EventStream;

use super::{input::WindowInputEvent, terminal::get_window_dimensions};

/// Window state for the editor
pub struct WindowState {
    /// Current cursor position
    ///
    /// Represented as a two element tuple with the x position as
    /// the first element and the y position as the second element.
    ///
    /// Initial value is `(0, 0)`.
    pub cursor_position: (u16, u16),
    /// Current window dimensions
    ///
    /// Represented as a two element tuple with the width as
    /// the first element and the height as the second element.
    pub dimensions: (u16, u16),
    /// Asynchronous event reader
    pub event_reader: EventStream,
    /// Vector of all window input events
    pub input_event_history: Vec<WindowInputEvent>,
}

impl WindowState {
    /// Creates a new `WindowState` value
    ///
    /// Example:
    ///
    /// ```
    /// use editor::state::WindowState;
    /// let state = WindowState::new();
    /// assert_eq!(state.input_event_history.len(), 0);
    /// ```
    pub fn new() -> WindowState {
        WindowState {
            cursor_position: (0, 0),
            dimensions: get_window_dimensions(),
            event_reader: EventStream::new(),
            input_event_history: Vec::<WindowInputEvent>::new(),
        }
    }
}

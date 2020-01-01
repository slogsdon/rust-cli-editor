/// Provides options for maintaining application state
use crossterm::event::EventStream;

use super::{input::WindowInputEvent, terminal::get_window_dimensions};

/// Window state for the editor
pub struct WindowState {
    /// Current content
    pub content: Vec<String>,
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
    pub fn new() -> Self {
        WindowState {
            content: Vec::new(),
            cursor_position: (0, 0),
            dimensions: get_window_dimensions(),
            event_reader: EventStream::new(),
            input_event_history: Vec::new(),
        }
    }

    pub fn last_content_line(&self) -> Option<&String> {
        self.content.last()
    }

    pub fn last_content_line_or<'a>(&'a self, default: &'a String) -> &'a String {
        match self.last_content_line() {
            Some(s) => s,
            None => default,
        }
    }

    pub fn push_input_event(&mut self, event: WindowInputEvent) {
        self.input_event_history.push(event)
    }

    pub fn last_input_event(&self) -> Option<&WindowInputEvent> {
        self.input_event_history.last()
    }

    pub fn update_cursor_position<F>(&mut self, f: F)
    where
        F: Fn((u16, u16)) -> (u16, u16),
    {
        self.cursor_position = f(self.cursor_position)
    }
}

impl Clone for WindowState {
    fn clone(&self) -> Self {
        WindowState {
            content: Vec::new(),
            cursor_position: self.cursor_position,
            dimensions: self.dimensions,
            event_reader: EventStream::new(),
            input_event_history: Vec::new(),
        }
    }
}

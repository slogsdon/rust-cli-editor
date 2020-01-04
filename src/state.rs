/// Provides options for maintaining application state
use crossterm::event::EventStream;

use super::{input::WindowInputEvent, terminal::get_window_dimensions};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum InputMode {
    NormalMode,
    InsertMode,
    CommandMode,
}

/// Window state for the editor
pub struct WindowState {
    pub command: String,
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
    pub filename: String,
    /// Vector of all window input events
    pub input_event_history: Vec<WindowInputEvent>,
    pub input_mode: InputMode,
    pub statusline_format: String,
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
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        WindowState {
            command: String::new(),
            content: Vec::new(),
            cursor_position: (0, 0),
            dimensions: get_window_dimensions(),
            event_reader: EventStream::new(),
            filename: String::new(),
            input_event_history: Vec::new(),
            input_mode: InputMode::NormalMode,
            statusline_format: String::from("[{mode}] L: {line}, C: {column}"),
        }
    }

    pub fn from_args(args: Vec<String>) -> Self {
        let mut state = WindowState::new();
        if let Some(filename) = args.get(1) {
            state.content = std::fs::read_to_string(filename)
                .unwrap_or(String::new())
                .lines()
                .map(String::from)
                .collect();
            state.filename = filename.to_string();
        }
        state
    }

    pub fn last_content_line(&self) -> Option<&String> {
        self.content.last()
    }

    pub fn last_content_line_or<'a>(&'a self, default: &'a str) -> &'a str {
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
            command: String::new(),
            content: Vec::new(),
            cursor_position: self.cursor_position,
            dimensions: self.dimensions,
            event_reader: EventStream::new(),
            filename: String::new(),
            input_event_history: Vec::new(),
            input_mode: self.input_mode,
            statusline_format: String::new(),
        }
    }
}

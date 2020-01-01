/// Provides ability to track application input
use crossterm::{
    event::{Event, KeyCode, KeyEvent, MouseEvent},
    Result,
};
use futures::{future::FutureExt, select, StreamExt};

use super::state::WindowState;

/// Window/user input event
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum WindowInputEvent {
    /// No operation
    ///
    /// Should occur only when no input is received before the
    /// event stream times out, etc.
    NoOp,
    /// User has requested to exit the application
    Exit,
    /// User has performed keyboard input
    KeyPress(KeyEvent),
    /// User has performed mouse input
    Mouse(MouseEvent),
    /// User has resized the terminal emulator window
    Resize(u16, u16),
}

impl From<Event> for WindowInputEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(e) => WindowInputEvent::KeyPress(e),
            Event::Mouse(e) => WindowInputEvent::Mouse(e),
            Event::Resize(x, y) => WindowInputEvent::Resize(x, y),
        }
    }
}

#[allow(clippy::unnecessary_mut_passed)]
pub async fn accept_window_input(state: &mut WindowState) -> Result<WindowInputEvent> {
    let mut event = state.event_reader.next().fuse();

    select! {
        maybe_event = event => {
            match maybe_event {
                Some(Ok(e)) => Ok(handle_window_input(state, WindowInputEvent::from(e))),
                Some(Err(e)) => Err(e),
                None => Ok(WindowInputEvent::NoOp),
            }
        }
    }
}

pub fn handle_window_input(
    state: &mut WindowState,
    mut event: WindowInputEvent,
) -> WindowInputEvent {
    state.push_input_event(event);

    match event {
        WindowInputEvent::Resize(width, height) => state.dimensions = (width, height),
        WindowInputEvent::KeyPress(key_event) => event = handle_key_press(state, key_event),
        _ => (),
    }

    event
}

fn handle_key_press(state: &mut WindowState, key_event: KeyEvent) -> WindowInputEvent {
    let default = String::new();
    let mut line = String::from(state.last_content_line_or(&default));
    let mut should_push = false;

    match key_event.code {
        KeyCode::Esc => {
            return WindowInputEvent::Exit;
        }
        KeyCode::Enter => {
            line = default;
            should_push = true;
            state.update_cursor_position(|(_, y)| (0, y + 1));
        }
        KeyCode::Char(ch) => {
            line.push(ch);
            state.update_cursor_position(|(x, y)| (x + 1, y));
        }
        _ => (),
    }

    if should_push || state.content.is_empty() {
        state.content.push(line);
    } else {
        let idx = state.content.len() - 1;
        std::mem::replace(&mut state.content[idx], line);
    }

    WindowInputEvent::KeyPress(key_event)
}

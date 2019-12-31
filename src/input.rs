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

impl WindowInputEvent {
    pub fn from_crossterm_event(event: Event) -> WindowInputEvent {
        match event {
            Event::Key(e) => {
                let event: KeyEvent = e;

                // TODO: remove with command mode
                if event.code == KeyCode::Esc {
                    return WindowInputEvent::Exit;
                }

                WindowInputEvent::KeyPress(event)
            }
            Event::Mouse(e) => WindowInputEvent::Mouse(e),
            Event::Resize(x, y) => WindowInputEvent::Resize(x, y),
        }
    }
}

pub async fn accept_window_input(state: &mut WindowState) -> Result<WindowInputEvent> {
    let mut event = state.event_reader.next().fuse();

    select! {
        maybe_event = event => {
            match maybe_event {
                Some(Ok(e)) => Ok(handle_window_input(state, WindowInputEvent::from_crossterm_event(e))),
                Some(Err(e)) => Err(e),
                None => Ok(WindowInputEvent::NoOp),
            }
        }
    }
}

pub fn handle_window_input(state: &mut WindowState, event: WindowInputEvent) -> WindowInputEvent {
    state.push_input_event(event);

    if let WindowInputEvent::Resize(width, height) = event {
        state.dimensions = (width, height);
    }

    event
}

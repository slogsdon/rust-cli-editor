/// Provides ability to track application input

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, MouseEvent},
    Result,
};
use futures::{future::FutureExt, select, StreamExt};

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
            },
            Event::Mouse(e) => WindowInputEvent::Mouse(e),
            Event::Resize(x, y) => WindowInputEvent::Resize(x, y),
        }
    }
}

pub async fn accept_window_input(reader: &mut EventStream) -> Result<WindowInputEvent> {
    let mut event = reader.next().fuse();

    select! {
        maybe_event = event => {
            match maybe_event {
                Some(Ok(e)) => Ok(WindowInputEvent::from_crossterm_event(e)),
                Some(Err(e)) => Err(e),
                None => Ok(WindowInputEvent::NoOp),
            }
        }
    }
}

pub fn handle_window_input(event: WindowInputEvent) {
    match event {
        WindowInputEvent::Exit => (),
        WindowInputEvent::NoOp => (),
        WindowInputEvent::Resize(_, _) => (),
        WindowInputEvent::Mouse(_) => (),
        WindowInputEvent::KeyPress(e) => {
            if let KeyCode::Char(c) = e.code {
                print!("{}", c);
            }

            if KeyCode::Enter == e.code {
                print!("\r\n");
            }
        }
    }
}

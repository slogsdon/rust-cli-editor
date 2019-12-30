use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, MouseEvent},
    Result,
};
use futures::{future::FutureExt, select, StreamExt};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum WindowInputEvent {
    NoOp,
    Exit,
    KeyPress(KeyEvent),
    Mouse(MouseEvent),
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

pub fn handle_window_input(_event: WindowInputEvent) {}

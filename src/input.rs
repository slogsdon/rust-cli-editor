use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, MouseEvent},
    Result,
};
use futures::{future::FutureExt, select, StreamExt};

#[derive(Clone, Copy, Debug)]
pub enum WindowInputEvent {
    NoOp,
    Exit,
    KeyPress(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub async fn accept_window_input(reader: &mut EventStream) -> Result<WindowInputEvent> {
    let mut event = reader.next().fuse();

    select! {
        maybe_event = event => {
            match maybe_event {
                Some(Ok(e)) => convert_crossterm_event_to_window_input_event(e),
                Some(Err(e)) => Err(e),
                None => Ok(WindowInputEvent::NoOp),
            }
        }
    }
}

pub fn handle_window_input(_event: WindowInputEvent) {}

fn convert_crossterm_event_to_window_input_event(event: Event) -> Result<WindowInputEvent> {
    match event {
        Event::Key(e) => {
            let event: KeyEvent = e;

            // TODO: remove with command mode
            if event.code == KeyCode::Esc {
                return Ok(WindowInputEvent::Exit);
            }

            Ok(WindowInputEvent::KeyPress(event))
        },
        Event::Mouse(e) => Ok(WindowInputEvent::Mouse(e)),
        Event::Resize(x, y) => Ok(WindowInputEvent::Resize(x, y)),
    }
}

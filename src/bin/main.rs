extern crate crossterm;
extern crate futures;

extern crate editor;

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, MouseEvent},
    Result,
};
use futures::{future::FutureExt, select, StreamExt};

use editor::terminal;

#[derive(Clone, Copy, Debug)]
enum WindowInputEvent {
    NoOp,
    Exit,
    KeyPress(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

async fn accept_window_input(reader: &mut EventStream) -> Result<WindowInputEvent> {
    let mut event = reader.next().fuse();

    select! {
        maybe_event = event => {
            match maybe_event {
                Some(Ok(event)) => {
                    match event {
                        Event::Key(e) => {
                            let event: KeyEvent = e;

                            if event.code == KeyCode::Esc {
                                return Ok(WindowInputEvent::Exit);
                            }

                            Ok(WindowInputEvent::KeyPress(event))
                        },
                        Event::Mouse(e) => Ok(WindowInputEvent::Mouse(e)),
                        Event::Resize(x, y) => Ok(WindowInputEvent::Resize(x, y)),
                    }
                }
                Some(Err(e)) => Err(e),
                None => Ok(WindowInputEvent::NoOp),
            }
        }
    }
}

fn handle_editor_event(_event: WindowInputEvent) {}

async fn main_loop() {
    let mut reader = EventStream::new();
    let mut history = Vec::<WindowInputEvent>::new();

    loop {
        let event = accept_window_input(&mut reader).await;

        match event {
            Err(e) => panic!("Error: {}", e),
            Ok(WindowInputEvent::NoOp) => continue,
            Ok(WindowInputEvent::Exit) => break,
            Ok(e) => {
                println!("Event: {:?}", e);
                history.push(e);
                handle_editor_event(e);
            },
        }
    }
}

fn main() -> Result<()> {
    terminal::setup_editor()?;
    async_std::task::block_on(main_loop());
    terminal::teardown_editor()
}

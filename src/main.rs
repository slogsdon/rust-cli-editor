extern crate crossterm;
extern crate futures;

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, MouseEvent},
    Result,
    terminal::{disable_raw_mode, enable_raw_mode}
};
use futures::{future::FutureExt, select, StreamExt};

#[derive(Clone, Copy, Debug)]
enum EditorEvent {
    NoOp,
    Exit,
    KeyPress(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

async fn accept_editor_event(reader: &mut EventStream) -> Result<EditorEvent> {
    let mut event = reader.next().fuse();

    select! {
        maybe_event = event => {
            match maybe_event {
                Some(Ok(event)) => {
                    match event {
                        Event::Key(e) => {
                            let event: KeyEvent = e;

                            if event.code == KeyCode::Esc {
                                return Ok(EditorEvent::Exit);
                            }

                            Ok(EditorEvent::KeyPress(event))
                        },
                        Event::Mouse(e) => Ok(EditorEvent::Mouse(e)),
                        Event::Resize(x, y) => Ok(EditorEvent::Resize(x, y)),
                    }
                }
                Some(Err(e)) => Err(e),
                None => Ok(EditorEvent::NoOp),
            }
        }
    }
}

fn hande_editor_event(_event: EditorEvent) {}

async fn main_loop() {
    let mut reader = EventStream::new();
    let mut history = Vec::<EditorEvent>::new();

    loop {
        let event = accept_editor_event(&mut reader).await;

        match event {
            Err(e) => panic!("Error: {}", e),
            Ok(EditorEvent::NoOp) => continue,
            Ok(EditorEvent::Exit) => break,
            Ok(e) => {
                println!("Event: {:?}", e);
                history.push(e);
                hande_editor_event(e);
            },
        }
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    enable_raw_mode()?;

    async_std::task::block_on(main_loop());

    disable_raw_mode()
}

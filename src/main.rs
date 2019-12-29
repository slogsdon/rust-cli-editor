extern crate crossterm;
extern crate futures;

use crossterm::{
    event::{Event, EventStream, KeyCode},
    Result,
    terminal::{disable_raw_mode, enable_raw_mode}
};
use futures::{future::FutureExt, select, StreamExt};

async fn main_loop() {
    let mut reader = EventStream::new();

    loop {
        let mut event = reader.next().fuse();

        select! {
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("hi c");
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    enable_raw_mode()?;

    async_std::task::block_on(main_loop());

    disable_raw_mode()
}

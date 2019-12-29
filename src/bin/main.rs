extern crate crossterm;
extern crate futures;

extern crate editor;

use crossterm::{
    event::{EventStream},
    Result,
};

use editor::input::{
    accept_window_input,
    handle_window_input,
    WindowInputEvent,
};
use editor::terminal;

fn main() -> Result<()> {
    terminal::setup_editor()?;
    async_std::task::block_on(main_loop());
    terminal::teardown_editor()
}

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
                handle_window_input(e);
            },
        }
    }
}

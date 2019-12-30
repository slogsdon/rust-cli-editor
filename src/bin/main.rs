use crossterm::{
    ErrorKind,
    event::{EventStream},
    Result,
};

use editor::{
    input::{
        accept_window_input,
        handle_window_input,
        WindowInputEvent,
    },
    state::WindowState,
    terminal,
};

fn main() -> Result<()> {
    terminal::setup_editor()?;
    async_std::task::block_on(main_loop())?;
    terminal::teardown_editor()
}

async fn main_loop() -> Result<()> {
    let mut error: Option<ErrorKind> = None;
    let mut reader = EventStream::new();
    let mut state = WindowState::new();

    loop {
        let event = accept_window_input(&mut reader).await;

        match event {
            Err(e) => {
                error = Some(e);
                break;
            },
            Ok(WindowInputEvent::NoOp) => continue,
            Ok(WindowInputEvent::Exit) => break,
            Ok(e) => {
                state.input_event_history.push(e);
                handle_window_input(e);
            },
        }
    }

    if let Some(e) = error {
        return Err(e);
    }

    Ok(())
}

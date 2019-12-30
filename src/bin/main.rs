use crossterm::Result;

use editor::{
    input::{accept_window_input, WindowInputEvent},
    state::WindowState,
    terminal,
    ui::render,
};

/// Application's entrypoint
///
/// Responsible for setting up and tearing down the editor's
/// environment.
fn main() -> Result<()> {
    terminal::setup_editor()?;
    async_std::task::block_on(main_loop())?;
    terminal::teardown_editor()
}

/// Main application loop
///
/// Responsible for handling input from the window/user and
/// controlling application state / UI changes when needed.
async fn main_loop() -> Result<()> {
    let mut state = WindowState::new();

    loop {
        match accept_window_input(&mut state).await {
            Err(e) => return Err(e),
            Ok(WindowInputEvent::Exit) => return Ok(()),
            Ok(WindowInputEvent::NoOp) => continue,
            Ok(_) => render(&mut state).await,
        }
    }
}

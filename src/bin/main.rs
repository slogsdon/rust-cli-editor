use crossterm::Result;

use editor::{
    input::{accept_window_input, WindowInputEvent},
    state::WindowState,
    terminal,
    ui::{reset_screen, render},
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

    // initial render to ensure screen is cleared and cursor
    // is placed at the intended first position
    reset_screen(&state)?;
    render(&mut state)?;

    loop {
        match accept_window_input(&mut state).await {
            Err(e) => return Err(e),
            Ok(WindowInputEvent::Exit) => return Ok(()),
            Ok(WindowInputEvent::NoOp) => continue,
            Ok(_) => {
                if let Err(e) = render(&mut state) {
                    return Err(e);
                }
            }
        }
    }
}

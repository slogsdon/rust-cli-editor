use crossterm::Result;

use editor::{
    input::{accept_window_input, WindowInputEvent},
    state::WindowState,
    terminal,
};

fn main() -> Result<()> {
    terminal::setup_editor()?;
    async_std::task::block_on(main_loop())?;
    terminal::teardown_editor()
}

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

async fn render(_state: &mut WindowState) {}

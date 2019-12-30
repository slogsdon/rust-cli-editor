use std::io::{stdout, Write};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    Result,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        size,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    }
};


// `EnterAlternateScreen` and `LeaveAlternateScreen` use a deprecated
// field of `std::err::Err`
#[allow(deprecated)]
pub fn setup_editor() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), EnableMouseCapture)?;
    enable_raw_mode()?;

    Ok(())
}


// `EnterAlternateScreen` and `LeaveAlternateScreen` use a deprecated
// field of `std::err::Err`
#[allow(deprecated)]
pub fn teardown_editor() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), DisableMouseCapture)?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

pub fn get_window_size() -> (u16, u16) {
    match size() {
        Ok((x, y)) => (x, y),
        Err(_) => (1, 1),
    }
}

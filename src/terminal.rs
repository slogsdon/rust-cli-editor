use std::io::{stdout, Write};

use crossterm::{
    execute,
    Result,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};


// `EnterAlternateScreen` and `LeaveAlternateScreen` use a deprecated
// field of `std::err::Err`
#[allow(deprecated)]
pub fn setup_editor() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    Ok(())
}


// `EnterAlternateScreen` and `LeaveAlternateScreen` use a deprecated
// field of `std::err::Err`
#[allow(deprecated)]
pub fn teardown_editor() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

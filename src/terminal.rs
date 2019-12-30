/// Interface for the terminal environment
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};
use std::io::{stdout, Write};

/// Prepares editor environment. Should be followed by
/// `teardown_editor`.
///
/// Example:
///
/// ```no_run
/// use crossterm::Result;
/// use editor::terminal::{setup_editor, teardown_editor};
///
/// fn main() -> Result<()> {
///     setup_editor()?;
///     // perform tasks
///     teardown_editor()
/// }
/// ```
#[allow(deprecated)]
// `EnterAlternateScreen` and `LeaveAlternateScreen` use a deprecated
// field of `std::err::Err`
pub fn setup_editor() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), EnableMouseCapture)?;
    enable_raw_mode()?;

    Ok(())
}

/// Resets editor environment to original state
///
/// Example:
///
/// ```no_run
/// use crossterm::Result;
/// use editor::terminal::{setup_editor, teardown_editor};
///
/// fn main() -> Result<()> {
///     setup_editor()?;
///     // perform tasks
///     teardown_editor()
/// }
/// ```
#[allow(deprecated)]
// `EnterAlternateScreen` and `LeaveAlternateScreen` use a deprecated
// field of `std::err::Err`
pub fn teardown_editor() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), DisableMouseCapture)?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

/// Gets current window dimensions
///
/// Return value is a two element tuple representing the width
/// as the first element and the height as the second element.
/// Default value is `(1, 1)`.
///
/// Example:
///
/// ```
/// use editor::terminal::get_window_dimensions;
/// let (width, height) = get_window_dimensions();
/// assert_ne!(width, 0);
/// assert_ne!(height, 0);
/// ```
pub fn get_window_dimensions() -> (u16, u16) {
    match size() {
        Ok((width, height)) => (width, height),
        Err(_) => (1, 1),
    }
}

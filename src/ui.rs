use crossterm::{cursor, queue, style, terminal, ErrorKind, Result};
use std::{
    fmt::Arguments,
    io::{stdout, Write},
};

use super::state::WindowState;

pub fn render(state: &WindowState) -> Result<()> {
    clear_screen()?;
    render_content(state)?;

    match stdout().flush() {
        Ok(v) => Ok(v),
        Err(e) => Err(ErrorKind::IoError(e)),
    }
}

fn render_content(state: &WindowState) -> Result<()> {
    let mut i: u16 = 0;
    for line in state.content.iter() {
        place_cursor((0, i))?;
        print_text(state, format_args!("{}", line))?;
        i += 1;
    }

    Ok(())
}

#[allow(deprecated)]
// `terminal::Clear` uses a deprecated field of `std::err::Err`
pub fn clear_screen() -> Result<()> {
    queue!(stdout(), terminal::Clear(terminal::ClearType::All))
}

#[allow(deprecated)]
// `cursor::MoveTo` uses a deprecated field of `std::err::Err`
pub fn place_cursor((x, y): (u16, u16)) -> Result<()> {
    queue!(stdout(), cursor::MoveTo(x, y))
}

#[allow(deprecated)]
// `style::Print` uses a deprecated field of `std::err::Err`
pub fn print_text(state: &WindowState, format_args: Arguments) -> Result<()> {
    queue!(stdout(), style::Print(format_args))?;
    place_cursor(state.cursor_position)
}

pub fn reset_screen(state: &WindowState) -> Result<()> {
    clear_screen()?;
    place_cursor(state.cursor_position)
}

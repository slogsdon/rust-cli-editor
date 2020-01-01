use crossterm::{cursor, queue, style, terminal, ErrorKind, Result};
use std::{
    convert::TryInto,
    fmt::Arguments,
    io::{stdout, Write},
};

use super::state::WindowState;

pub fn render(state: &WindowState) -> Result<()> {
    clear_screen()?;
    render_content(state)?;
    render_statusline(state)?;

    match stdout().flush() {
        Ok(_) => Ok(()),
        Err(e) => Err(ErrorKind::IoError(e)),
    }
}

// TODO: Account for viewable space / bounds from window dimensions
fn render_content(state: &WindowState) -> Result<()> {
    for (i, line) in state.content.iter().enumerate() {
        place_cursor((0, i.try_into().unwrap()))?;
        print_text(state, format_args!("{}", line))?;
    }

    Ok(())
}

fn render_statusline(state: &WindowState) -> Result<()> {
    let (_, height) = state.dimensions;
    place_cursor((0, height - 1))?;
    let (x, y) = state.cursor_position;
    let line: String = format!("{}", y + 1);
    let column: String = format!("{}", x + 1);
    print_text(
        state,
        format_args!(
            "{}",
            state
                .statusline_format
                .replace("{line}", line.as_str())
                .replace("{column}", column.as_str())
        ),
    )?;

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

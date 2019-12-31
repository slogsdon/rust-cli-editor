use crossterm::event::{KeyCode,KeyEvent};
use crossterm::{cursor, queue, style, terminal, ErrorKind, Result};
use std::{
    fmt::Arguments,
    io::{stdout, Write},
};

use super::{input::WindowInputEvent, state::WindowState};

pub fn render(state: &mut WindowState) -> Result<()> {
    let event: &WindowInputEvent;
    let state_clone = state.clone();

    match state_clone.last_input_event() {
        None => return Ok(()),
        Some(e) => event = e,
    }

    match event {
        WindowInputEvent::Exit => (),
        WindowInputEvent::NoOp => (),
        // TODO: render mouse event
        WindowInputEvent::Mouse(_) => (),
        // TODO: render resize event
        WindowInputEvent::Resize(_,_) => (),
        WindowInputEvent::KeyPress(key_event) => {
            let _ = render_key_press(state, key_event);
        }
    }

    match stdout().flush() {
        Ok(v) => Ok(v),
        Err(e) => Err(ErrorKind::IoError(e)),
    }
}

fn render_key_press(state: &mut WindowState, key_event: &KeyEvent) -> Result<()> {
    let mut c = '\0';

    match key_event.code {
        KeyCode::Enter => {
            c = '\r';
            let (x,y) = state.cursor_position;
            state.cursor_position = (x, y+1);
        }
        KeyCode::Char(ch) => {
            c = ch;
            let (x,y) = state.cursor_position;
            state.cursor_position = (x+1, y);
        }
        _ => (),
    }

    print_text(state, format_args!("{}", c))
}

#[allow(deprecated)]
// `terminal::Clear` uses a deprecated field of `std::err::Err`
pub fn clear_screen() -> Result<()> {
    queue!(stdout(), terminal::Clear(terminal::ClearType::All))
}

#[allow(deprecated)]
// `cursor::MoveTo` uses a deprecated field of `std::err::Err`
pub fn place_cursor(state: &WindowState) -> Result<()> {
    let (x, y) = state.cursor_position;
    queue!(stdout(), cursor::MoveTo(x, y))
}

#[allow(deprecated)]
// `style::Print` uses a deprecated field of `std::err::Err`
pub fn print_text(state: &WindowState, format_args: Arguments) -> Result<()> {
    queue!(stdout(), style::Print(format_args))?;
    place_cursor(state)
}

pub fn reset_screen(state: &WindowState) -> Result<()> {
    clear_screen()?;
    place_cursor(state)
}

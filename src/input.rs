/// Provides ability to track application input
use crossterm::{
    event::{Event, KeyCode, KeyEvent, MouseEvent},
    Result,
};
use futures::{future::FutureExt, select, StreamExt};

use super::state::{InputMode, WindowState};

/// Window/user input event
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum WindowInputEvent {
    /// No operation
    ///
    /// Should occur only when no input is received before the
    /// event stream times out, etc.
    NoOp,
    /// User has requested to exit the application
    Exit,
    /// User has performed keyboard input
    KeyPress(KeyEvent),
    /// User has performed mouse input
    Mouse(MouseEvent),
    /// User has resized the terminal emulator window
    Resize(u16, u16),
    ChangeMode(InputMode, InputMode),
    WriteFile,
}

impl From<Event> for WindowInputEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(e) => WindowInputEvent::KeyPress(e),
            Event::Mouse(e) => WindowInputEvent::Mouse(e),
            Event::Resize(x, y) => WindowInputEvent::Resize(x, y),
        }
    }
}

#[allow(clippy::unnecessary_mut_passed)]
pub async fn accept_window_input(state: &mut WindowState) -> Result<WindowInputEvent> {
    let mut event = state.event_reader.next().fuse();

    select! {
        maybe_event = event => {
            match maybe_event {
                Some(Ok(e)) => Ok(handle_window_input(state, WindowInputEvent::from(e))),
                Some(Err(e)) => Err(e),
                None => Ok(WindowInputEvent::NoOp),
            }
        }
    }
}

pub fn handle_window_input(
    state: &mut WindowState,
    mut event: WindowInputEvent,
) -> WindowInputEvent {
    state.push_input_event(event);

    match event {
        WindowInputEvent::Resize(width, height) => state.dimensions = (width, height),
        WindowInputEvent::KeyPress(key_event) => event = handle_key_press(state, key_event),
        _ => (),
    }

    if let WindowInputEvent::ChangeMode(_, to) = event {
        state.input_mode = to;
    }

    event
}

fn handle_key_press(state: &mut WindowState, key_event: KeyEvent) -> WindowInputEvent {
    match state.input_mode {
        InputMode::NormalMode => handle_key_press_normal(state, key_event),
        InputMode::InsertMode => handle_key_press_insert(state, key_event),
        InputMode::CommandMode => handle_key_press_command(state, key_event),
    }
}

fn handle_key_press_normal(state: &mut WindowState, key_event: KeyEvent) -> WindowInputEvent {
    match key_event.code {
        KeyCode::Char(':') => {
            WindowInputEvent::ChangeMode(state.input_mode, InputMode::CommandMode)
        }
        KeyCode::Char('i') => WindowInputEvent::ChangeMode(state.input_mode, InputMode::InsertMode),
        KeyCode::Char('h') => {
            state.update_cursor_position(|(x, y)| (if x > 0 { x - 1 } else { 0 }, y));
            WindowInputEvent::KeyPress(key_event)
        }
        KeyCode::Char('k') => {
            state.update_cursor_position(|(x, y)| (x, if y > 0 { y - 1 } else { 0 }));
            WindowInputEvent::KeyPress(key_event)
        }
        KeyCode::Char('j') => {
            state.update_cursor_position(|(x, y)| (x, y + 1));
            WindowInputEvent::KeyPress(key_event)
        }
        KeyCode::Char('l') => {
            state.update_cursor_position(|(x, y)| (x + 1, y));
            WindowInputEvent::KeyPress(key_event)
        }
        _ => WindowInputEvent::KeyPress(key_event),
    }
}

fn handle_key_press_insert(state: &mut WindowState, key_event: KeyEvent) -> WindowInputEvent {
    let default = String::new();
    let mut line = String::from(state.last_content_line_or(&default));
    let mut should_push = false;

    match key_event.code {
        KeyCode::Esc => {
            return WindowInputEvent::ChangeMode(state.input_mode, InputMode::NormalMode)
        }
        KeyCode::Enter => {
            line = default;
            should_push = true;
            state.update_cursor_position(|(_, y)| (0, y + 1));
        }
        KeyCode::Char(ch) => {
            line.push(ch);
            state.update_cursor_position(|(x, y)| (x + 1, y));
        }
        _ => (),
    }

    if should_push || state.content.is_empty() {
        state.content.push(line);
    } else {
        let idx = state.content.len() - 1;
        std::mem::replace(&mut state.content[idx], line);
    }

    state.has_content_changed = true;
    WindowInputEvent::KeyPress(key_event)
}

fn handle_key_press_command(state: &mut WindowState, key_event: KeyEvent) -> WindowInputEvent {
    if let KeyCode::Esc = key_event.code {
        state.command = String::new();
        return WindowInputEvent::ChangeMode(state.input_mode, InputMode::NormalMode);
    }

    if let KeyCode::Enter = key_event.code {
        return parse_command(state);
    }

    if let KeyCode::Char(c) = key_event.code {
        state.command.push(c);
    }

    WindowInputEvent::KeyPress(key_event)
}

fn parse_command(state: &mut WindowState) -> WindowInputEvent {
    if state.command.eq("q") {
        return WindowInputEvent::Exit;
    }

    if state.command.eq("w") && state.has_content_changed {
        return WindowInputEvent::WriteFile;
    }

    state.command = String::new();
    WindowInputEvent::ChangeMode(state.input_mode, InputMode::NormalMode)
}

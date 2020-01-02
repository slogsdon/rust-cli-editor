use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use editor::{
    input::{handle_window_input, WindowInputEvent},
    state::{InputMode, WindowState},
};

#[test]
fn window_input_event_from_crossterm_event_handles_key_event() {
    let crossterm_event = KeyEvent::from(KeyCode::Char('f'));
    let event = WindowInputEvent::from(Event::Key(crossterm_event));

    assert_eq!(WindowInputEvent::KeyPress(crossterm_event), event);
}

#[test]
fn window_input_event_from_crossterm_event_handles_mouse_event() {
    let crossterm_event = MouseEvent::ScrollDown(1, 1, KeyModifiers::empty());
    let event = WindowInputEvent::from(Event::Mouse(crossterm_event));

    assert_eq!(WindowInputEvent::Mouse(crossterm_event), event);
}

#[test]
fn window_input_event_from_crossterm_event_handles_resize_event() {
    let crossterm_event = Event::Resize(1, 1);
    let event = WindowInputEvent::from(crossterm_event);

    assert_eq!(WindowInputEvent::Resize(1, 1), event);
}

#[test]
fn handle_window_input_updates_key_event_with_escape_to_exit_insert_command_modes() {
    let crossterm_event = KeyEvent::from(KeyCode::Esc);
    let event = WindowInputEvent::from(Event::Key(crossterm_event));
    let mut state = WindowState::new();

    assert_eq!(WindowInputEvent::KeyPress(crossterm_event), event);

    state.input_mode = InputMode::InsertMode;

    assert_eq!(
        WindowInputEvent::ChangeMode(InputMode::InsertMode, InputMode::NormalMode),
        handle_window_input(&mut state, event)
    );

    state.input_mode = InputMode::CommandMode;

    assert_eq!(
        WindowInputEvent::ChangeMode(InputMode::CommandMode, InputMode::NormalMode),
        handle_window_input(&mut state, event)
    );
}

#[test]
fn handle_window_input_updates_key_event_with_enter_on_q_command() {
    let crossterm_event = KeyEvent::from(KeyCode::Enter);
    let event = WindowInputEvent::from(Event::Key(crossterm_event));
    let mut state = WindowState::new();
    state.input_mode = InputMode::CommandMode;
    state.command = String::from("q");

    assert_eq!(WindowInputEvent::KeyPress(crossterm_event), event);
    assert_eq!(
        WindowInputEvent::Exit,
        handle_window_input(&mut state, event)
    );
}

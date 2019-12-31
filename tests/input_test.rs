use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use editor::input::WindowInputEvent;

#[test]
fn window_input_event_from_crossterm_event_handles_key_event() {
    let crossterm_event = KeyEvent::from(KeyCode::Char('f'));
    let event = WindowInputEvent::from(Event::Key(crossterm_event));

    assert_eq!(WindowInputEvent::KeyPress(crossterm_event), event);
}

#[test]
fn window_input_event_from_crossterm_event_handles_key_event_with_escape() {
    let crossterm_event = KeyEvent::from(KeyCode::Esc);
    let event = WindowInputEvent::from(Event::Key(crossterm_event));

    assert_eq!(WindowInputEvent::Exit, event);
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

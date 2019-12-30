use editor::state::WindowState;

#[test]
fn new() {
    let state = WindowState::new();
    let (cx, cy) = state.cursor_position;

    assert_eq!(state.input_event_history.len(), 0);
    assert_eq!(cx, 0);
    assert_eq!(cy, 0);
}

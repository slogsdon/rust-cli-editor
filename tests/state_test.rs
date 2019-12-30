use editor::state::WindowState;

#[test]
fn new() {
    let state = WindowState::new();

    assert_eq!(state.input_event_history.len(), 0);
}

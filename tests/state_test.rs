use editor::state::{WindowState};

#[test]
fn new() {
    let state = WindowState::new();

    assert_eq!(state.event_history.len(), 0);
}

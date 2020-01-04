use editor::state::WindowState;

#[test]
fn new() {
    let state = WindowState::new();
    let (cx, cy) = state.cursor_position;

    assert_eq!(state.input_event_history.len(), 0);
    assert_eq!(cx, 0);
    assert_eq!(cy, 0);
}

#[test]
fn from_args_can_read_file() {
    let state = WindowState::from_args(vec![
        String::from("editor"),
        String::from("./tests/fixtures/file.txt"),
    ]);
    assert_ne!(state.content.len(), 0);
    assert_eq!(state.content, vec!["a", "b"]);
    assert_eq!(state.filename, "./tests/fixtures/file.txt");
}

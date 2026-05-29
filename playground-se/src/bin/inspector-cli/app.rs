#[derive(Debug, Default)]
pub struct App {
    // Input state
    // TODO: updated to screens
    pub input_mode: InputMode,
    pub file_path: String,
    pub cursor_position: usize,
}

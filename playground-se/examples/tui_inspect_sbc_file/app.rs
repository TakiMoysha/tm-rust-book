use std::path::PathBuf;

use ratatui::widgets::ListState;

use playground_se::{helpers::parse_sbc_definitions, types::Definition};

#[derive(Debug, Default)]
pub struct App {
    // Input state
    // TODO: updated to screens
    pub input_mode: InputMode,
    pub file_path: String,
    pub cursor_position: usize,

    // Data state
    pub definitions: Vec<Definition>,
    pub parse_error: Option<String>,

    // UI state
    pub msg: Option<String>,
    pub list_state: ListState,
    pub should_quit: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
}

impl App {
    pub fn new() -> Self {
        Self {
            msg: Some("Hello world!".into()),
            ..Self::default()
        }
    }

    pub fn with_file(path: &str) -> Self {
        let mut app = Self::new();
        app.file_path = path.to_string();
        app.cursor_position = path.len();
        app.try_parse_file();
        app
    }

    pub fn notify(&mut self, msg: impl AsRef<str>) {
        self.msg = Some(msg.as_ref().into());
    }

    /// Switch to Insert mode
    pub fn enter_insert_mode(&mut self) {
        self.input_mode = InputMode::Insert;
    }

    /// Switch to Normal mode and attempt to parse
    pub fn enter_normal_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.try_parse_file();
    }

    /// Try to parse the current file path
    pub fn try_parse_file(&mut self) {
        self.parse_error = None;
        self.definitions.clear();
        self.list_state.select(None);

        if self.file_path.is_empty() {
            return;
        }

        let path = PathBuf::from(&self.file_path);

        if !path.exists() || !path.is_file() {
            self.parse_error = Some(format!("Invalid file: {}", self.file_path));
            return;
        }

        if let Ok(sbc) = parse_sbc_definitions(&path) {
            self.definitions = sbc;
            if !self.definitions.is_empty() {
                self.list_state.select(Some(0));
            }
        } else {
            self.parse_error = Some(format!("Parse error: {}", self.file_path));
        }
    }

    /// Get currently selected definition
    pub fn selected_definition(&self) -> Option<&Definition> {
        self.list_state
            .selected()
            .and_then(|idx| self.definitions.get(idx))
    }

    /// Navigation
    pub fn select_next(&mut self) {
        if self.definitions.is_empty() {
            return;
        }
        let idx = self.list_state.selected().unwrap_or(0);
        let new_idx = (idx + 1).min(self.definitions.len() - 1);
        self.list_state.select(Some(new_idx));
    }

    pub fn select_previous(&mut self) {
        if self.definitions.is_empty() {
            return;
        }
        let idx = self.list_state.selected().unwrap_or(0);
        let new_idx = idx.saturating_sub(1);
        self.list_state.select(Some(new_idx));
    }

    pub fn select_first(&mut self) {
        if !self.definitions.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        if !self.definitions.is_empty() {
            self.list_state.select(Some(self.definitions.len() - 1));
        }
    }

    /// Input handling
    pub fn insert_char(&mut self, c: char) {
        self.file_path.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.file_path.remove(self.cursor_position);
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position < self.file_path.len() {
            self.file_path.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_position = (self.cursor_position + 1).min(self.file_path.len());
    }

    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.file_path.len();
    }

    pub fn clear_input(&mut self) {
        self.file_path.clear();
        self.cursor_position = 0;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

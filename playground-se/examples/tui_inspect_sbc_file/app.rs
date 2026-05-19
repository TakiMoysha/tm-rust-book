use std::fs;
use std::path::{Path, PathBuf};

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

    // Float suggestions window state
    pub show_suggestions: bool,
    pub suggestions: Vec<SuggestionItem>,
    pub selected_suggestion: usize,
    pub suggestion_input: String,
}

#[derive(Debug, Clone)]
pub struct SuggestionItem {
    pub name: String,
    pub is_dir: bool,
    pub is_sbc: bool,
    pub full_path: PathBuf,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
}

impl App {
    pub fn new() -> Self {
        Self { ..Self::default() }
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

    // === Suggestions float window methods ===

    /// Open suggestions window
    pub fn open_suggestions(&mut self) {
        self.show_suggestions = true;
        self.suggestion_input.clone_from(&self.file_path);
        self.selected_suggestion = 0;
        self.refresh_suggestions();
    }

    /// Close suggestions window without applying
    pub fn close_suggestions(&mut self) {
        self.show_suggestions = false;
        self.suggestions.clear();
        self.selected_suggestion = 0;
    }

    /// Apply selected suggestion and close
    pub fn accept_suggestion(&mut self) {
        if let Some(item) = self.suggestions.get(self.selected_suggestion) {
            self.suggestion_input = item.full_path.to_string_lossy().to_string();
        }
        self.file_path.clone_from(&self.suggestion_input);
        self.cursor_position = self.file_path.len();
        self.close_suggestions();
        self.enter_normal_mode();
    }

    /// Refresh suggestions based on current input
    pub fn refresh_suggestions(&mut self) {
        self.suggestions.clear();
        self.selected_suggestion = 0;

        let path = Path::new(&self.suggestion_input);
        let (dir_to_read, prefix) = if self.suggestion_input.is_empty() {
            (PathBuf::from("."), String::new())
        } else if path.is_dir() && self.suggestion_input.ends_with('/') {
            (path.to_path_buf(), String::new())
        } else {
            let parent = path.parent().unwrap_or(Path::new("."));
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            (parent.to_path_buf(), file_name)
        };

        if let Ok(entries) = fs::read_dir(&dir_to_read) {
            let mut items: Vec<SuggestionItem> = entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = entry.file_type().ok()?.is_dir();
                    let is_sbc = !is_dir && name.ends_with(".sbc");

                    // Skip hidden files
                    if name.starts_with('.') {
                        return None;
                    }

                    // Filter by prefix if any
                    if !prefix.is_empty()
                        && !name.to_lowercase().starts_with(&prefix.to_lowercase())
                    {
                        return None;
                    }

                    let full_path = if dir_to_read == Path::new(".") {
                        entry.path()
                    } else {
                        dir_to_read.join(entry.file_name())
                    };

                    Some(SuggestionItem {
                        name: if is_dir { format!("{}/", name) } else { name },
                        is_dir,
                        is_sbc,
                        full_path,
                    })
                })
                .collect();

            // Sort: directories first, then alphabetically
            items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            self.suggestions = items;
        }
    }

    /// Navigate down in suggestions
    pub fn select_next_suggestion(&mut self) {
        if !self.suggestions.is_empty() {
            self.selected_suggestion =
                (self.selected_suggestion + 1).min(self.suggestions.len() - 1);
        }
    }

    /// Navigate up in suggestions
    pub fn select_prev_suggestion(&mut self) {
        self.selected_suggestion = self.selected_suggestion.saturating_sub(1);
    }

    /// Update suggestion input from char
    pub fn suggestion_insert_char(&mut self, c: char) {
        self.suggestion_input.push(c);
        self.refresh_suggestions();
    }

    /// Backspace in suggestions
    pub fn suggestion_backspace(&mut self) {
        if !self.suggestion_input.is_empty() {
            self.suggestion_input.pop();
            self.refresh_suggestions();
        }
    }

    /// Navigate into selected directory
    pub fn suggestion_enter_dir(&mut self) {
        if let Some(item) = self.suggestions.get(self.selected_suggestion) {
            if item.is_dir {
                self.suggestion_input = format!("{}/", item.full_path.to_string_lossy());
                self.refresh_suggestions();
            }
        }
    }
}

#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_index: usize,
    pub file_name: String,
    pub is_modified: bool,
}

impl DocumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("*")
        } else {
            String::new()
        }
    }

    pub fn line_count_to_string(&self) -> String {
        format!("/{}", self.total_lines)
    }

    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}{}",
            self.current_line_index.saturating_add(1),
            self.total_lines
        )
    }
}

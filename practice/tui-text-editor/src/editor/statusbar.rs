use std::io::Error;

use super::{
    document_status::DocumentStatus, size::Size, terminal::Terminal, ui_components::UIComponent,
};

#[derive(Default)]
pub struct StatusBar {
    size: Size,
    current_status: DocumentStatus,
    should_redraw: bool,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.should_redraw = true;
        }
    }
}

impl UIComponent for StatusBar {
    fn set_redraw_flag(&mut self, value: bool) {
        self.should_redraw = value;
    }

    fn is_should_redraw(&self) -> bool {
        self.should_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin_row: usize) -> Result<(), Error> {
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();

        let beginning = format!(
            "{} - {line_count}{modified_indicator}",
            self.current_status.file_name
        );

        let position_indicator = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator:>remainder_len$}");

        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };
        Terminal::print_inverted_row(origin_row, &to_print)?;

        Ok(())
    }
}

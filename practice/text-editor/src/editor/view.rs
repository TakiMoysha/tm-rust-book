mod buffer;

use buffer::Buffer;

use super::terminal::{Position, Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum ViewError {
    Undefined(String),
}

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Default, Debug)]
pub struct View {
    buffer: Buffer,
    size: Size,
    is_redraw: bool,
}

impl View {
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self {
            buffer,
            ..Default::default()
        }
    }

    pub(crate) fn resize(&mut self, to: Size) {
        self.size = to;
        self.is_redraw = true;
    }

    fn render_line(at: usize, text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { col: 0, row: at })?;
        Terminal::clear_line()?;
        Terminal::print(text)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.is_redraw {
            return Ok(());
        }

        let Size { height, width } = Terminal::size()?;
        if height == 0 || width == 0 {
            return Ok(());
        }

        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.data.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line)?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        self.is_redraw = false;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = buffer::Buffer::load(file_name) {
            self.buffer = buffer;
        };
    }

    fn draw_welcome() -> Result<(), Error> {
        let welcome_msg = Self::build_welcome(Terminal::size()?.width);
        Terminal::print(&welcome_msg)?;
        Ok(())
    }

    fn build_welcome(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_msg = format!("{NAME} -- v{VERSION}");
        let len = welcome_msg.len();
        if width <= len {
            return "~".to_string();
        }

        let padding = width.saturating_sub(len).saturating_sub(1) / 2;

        let mut full_msg = format!("~{}{}", " ".repeat(padding), welcome_msg);
        full_msg.truncate(width);
        full_msg
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("DEB:{current_row}")?;
            if current_row == height / 3 {
                Self::draw_welcome()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        todo!()
    }
}

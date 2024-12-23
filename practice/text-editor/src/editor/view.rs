use super::{
    buffer,
    terminal::{Size, Terminal},
};
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
    buffer: buffer::Buffer,
}

impl View {
    pub fn new(buffer: buffer::Buffer) -> Self {
        Self { buffer }
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::draw_welcome()?;
        } else {
            self.draw_buffer()?;
        }
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = buffer::Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }

    fn draw_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.data.get(current_row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    fn draw_welcome() -> Result<(), Error> {
        let mut welcome_msg = format!("{NAME} -- v{VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_msg.len();

        let padding = width.saturating_sub(len) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1) as usize);
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        Terminal::print(&welcome_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("DEB:{current_row}")?;
            if current_row == height / 3 {
                Self::draw_welcome()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn demo_draw(&self) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;
        Terminal::print(format!("{}", "=".repeat(width as usize)).as_str());
        Ok(())
    }
}

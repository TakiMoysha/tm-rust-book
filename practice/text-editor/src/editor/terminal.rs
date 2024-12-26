use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    Command,
};

#[derive(Clone, Debug, Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternative_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn init() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternative_screen()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn enter_alternative_screen() -> Result<(), Error> {
        Self::queue_command(crossterm::terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternative_screen() -> Result<(), Error> {
        Self::queue_command(crossterm::terminal::LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn move_caret_to(pos: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        let (width, height) = (width as usize, height as usize);
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn print_line(at: usize, text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row: at })?;
        Self::clear_line()?;
        Self::print(text)?;
        Ok(())
    }
}

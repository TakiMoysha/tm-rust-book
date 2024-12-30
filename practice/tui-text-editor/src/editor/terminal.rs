use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Attribute, Print},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        SetTitle,
    },
    Command,
};

use super::{position::Position, size::Size};

pub struct Terminal;

impl Terminal {
    // ======================================= LIFECYCLE
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternative_screen()?;
        Self::enable_line_wrap()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn init() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternative_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    // ======================================== SCREEN
    pub fn enter_alternative_screen() -> Result<(), Error> {
        Self::queue_command(crossterm::terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternative_screen() -> Result<(), Error> {
        Self::queue_command(crossterm::terminal::LeaveAlternateScreen)?;
        Ok(())
    }

    // ======================================== CLEAR
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    // ======================================== CURSOR
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

    // ======================================== SIZE
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn print_row(at: usize, text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row: at })?;
        Self::clear_line()?;
        Self::print(text)?;
        Ok(())
    }

    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error> {
        let width = Self::size()?.width;
        Self::print_row(
            row,
            &format!(
                "{}{:width$.width$}{}",
                Attribute::Reverse,
                line_text,
                Attribute::Reset,
            ),
        )
    }

    // ======================================== RENDERING
    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }

    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }

    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        let (width, height) = (width as usize, height as usize);
        Ok(Size { height, width })
    }
    // ======================================== EXEC
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}

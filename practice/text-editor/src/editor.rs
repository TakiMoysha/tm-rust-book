mod terminal;

use std::env;
use std::io::{self, stdout, Error};

use crossterm::cursor::{self};
use crossterm::event::read;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, ExecutableCommand};

use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let res = self.repl();
        Terminal::terminate().unwrap();
        res.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true
                }
                KeyCode::Char('d') if *modifiers == KeyModifiers::CONTROL => {
                    self.demo_draw();
                }
                _ => println!("{:?} \r", *code),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            self.demo_draw()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn demo_draw(&self) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;
        Terminal::print(format!("{}", "=".repeat(width as usize)).as_str());
        Ok(())
    }

    fn draw_welcome_msg() -> Result<(), Error> {
        let mut welcome_msg = format!("{NAME} -- v{VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_msg.len();

        let padding = width.saturating_sub(len as u16) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1) as usize);
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width as usize);
        Terminal::print(welcome_msg.as_str())?;
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
            Terminal::print(format!("DEB:{}", current_row).as_str())?;
            if current_row == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}

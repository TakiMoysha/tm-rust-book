mod editor_command;
mod terminal;
mod view;

use std::env;
use std::io::Error;
use std::panic::{set_hook, take_hook};

use crossterm::event::{read, KeyEventKind};
use crossterm::event::{Event, KeyEvent};

use editor_command::EditorCommand;
use terminal::Terminal;
use view::View;

use crate::debug_panic;

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye!\r\n");
        }
    }
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_panic_hook = take_hook();
        set_hook(Box::new(move |info| {
            let _ = Terminal::terminate();
            current_panic_hook(info);
        }));
        Terminal::init()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event).unwrap(),
                Err(err) => debug_panic!(format!("Could not read from stdin: {err}")),
            };
        }
    }

    fn evaluate_event(&mut self, event: Event) -> Result<(), Error> {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => debug_panic!(format!("Could not handle command: {err}")),
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.get_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

#[cfg(test)]
mod editor_tests {
    use super::*;
}

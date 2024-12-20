mod editor {
    use std::io::{self, stdout};

    use crossterm::event::read;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal::disable_raw_mode;
    use crossterm::terminal::enable_raw_mode;
    use crossterm::execute;
    use crossterm::terminal::{Clear, ClearType};

    fn print_symbol(b: KeyCode) {
        println!("Binary: {} \r", b);
        // if c.is_control() {
        //     println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        // } else {
        //     println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?} \r", b, c);
        // }
    }

    pub struct Editor {
        should_quit: bool,
    }

    impl Editor {
        pub fn default() -> Self {
            Editor { should_quit: false }
        }

        pub fn run(&mut self) {
            Self::init().unwrap();
            let res = self.repl();
            Self::terminate().unwrap();
            res.unwrap();
        }

        fn init() -> Result<(), std::io::Error> {
            enable_raw_mode()?;
            Self::clear_screen()
        }

        fn terminate() -> Result<(), std::io::Error> {
            disable_raw_mode()
        }

        fn clear_screen() -> Result<(), std::io::Error> {
            let mut stdout = stdout();
            execute!(stdout, Clear(ClearType::All))
        }

        fn repl(&mut self) -> io::Result<()> {
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
            if let Event::Key(KeyEvent {code, modifiers, .. }) = event {
                match code {
                    KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                    _ => print_symbol(*code),
                }
            }
        }

        fn refresh_screen(&self) -> Result<(), std::io::Error> {
            if self.should_quit {
                Self::clear_screen()?;
                println!("Goodbye!\r\n");

            }
            Ok(())
        }
    }
}

fn main() {
    editor::Editor::default().run();
}

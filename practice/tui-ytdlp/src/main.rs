use std::path::PathBuf;

use clap::{Arg, Parser};
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

/// use cases:
/// - i want download with 'standard' preset (quality: 1080p, )
/// - use dir as library directory (library file in current dir or xdg_config)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_args = clap::Command::new("tm-ytlib")
        .author("takimoysha")
        .version("0.1.0")
        .about("TUI for my pipeline of yt-dlp (video download and processing)")
        .args([
            Arg::new("config")
                .short('c')
                .required(false)
                .default_value("default"),
            Arg::new("debug"),
        ])
        .after_help("example: tm-ytlib --config ~/.config/tm-ytlib/config.toml")
        .get_matches();

    let mut app_state = AppState::default();
    let mut tui = tui::TuiRuntime::enter()?;

    tui.run();
    // ==================================================== finish
    Ok(())
}

#[derive(Debug)]
pub struct AppConfig {
    pub opts: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            opts: vec!["-S \"+res:1080p\"".to_string()],
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub config: AppConfig,

    pub show_help: bool,
    pub is_exit: bool,
}

mod tui {
    use crossterm::execute;
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use ratatui::{Frame, Terminal, backend::CrosstermBackend, widgets::Widget};
    use ratatui::{buffer::Buffer, widgets};
    use std::{
        error::Error,
        io::{self, Stdout},
        result::Result,
    };

    use crate::{AppConfig, AppState};

    pub struct TuiRuntime {
        pub terminal: Terminal<CrosstermBackend<Stdout>>,

        pub state: AppState,
    }

    impl TuiRuntime {
        /// setup environment (terminal) for TUI
        pub fn enter(config: &AppConfig) -> Result<Self, Box<dyn Error>> {
            enable_raw_mode()?;
            let mut stdout = std::io::stdout();

            // alternate screen - separate buffer, (take all space for tui)
            execute!(
                stdout,
                crossterm::terminal::EnterAlternateScreen,
                crossterm::event::EnableMouseCapture
            )?;

            let backend = CrosstermBackend::new(stdout);
            let terminal = Terminal::new(backend)?;

            if let Some(value) = config.opts.get(0) {}

            Ok(Self {
                terminal,
                state: AppState::default(),
            })
        }

        pub fn draw<F>(&mut self, f: F) -> std::io::Result<()>
        where
            F: FnOnce(&mut Frame),
        {
            self.terminal.draw(f)?;
            Ok(())
        }

        pub(crate) fn run(&self) -> Result<(), Box<dyn Error>> {
            while !self.state.is_exit {
                self.draw(|frame| self.render_ui(frame))?;

                // if let Event::Key(key) = event::read()? {
                //     match key.code {
                //         KeyCode::Char('?') => self.state.show_help = !self.state.show_help,
                //         KeyCode::Char('i') => {}
                //         _ => (),
                //     }
                //
                //     if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                //         // check processes and finalize them
                //         self.state.is_exit = true;
                //     }
                // }
            }

            Ok(())
        }

        pub fn render_ui(&self, frame: &mut Frame) {
            let display_text = if self.state.show_help {
                format!("MODAL\nState: {:#?}", self.state);
            } else {
                format!("MAIN Screen\n Press '?' for help. State: {:#?}", app);
            };

            todo!()
        }

        pub fn handle_events(&mut self) -> io::Result<()> {
            todo!()
        }
    }

    impl Widget for &TuiRuntime {
        fn render(self, area: ratatui::Layout::React, buf: &mut Buffer) {
            buf.render_widget(
                widgets::Paragraph::new("Hello World!")
                    .alignment(ratatui::layout::Alignment::Center),
                area(),
            );
        }
    }

    impl Drop for TuiRuntime {
        fn drop(&mut self) {
            disable_raw_mode()
                .map_err(|_| ())
                .expect("failed to disable raw mode, with unexpected error");

            execute!(
                self.terminal.backend_mut(),
                crossterm::terminal::LeaveAlternateScreen,
                crossterm::event::DisableMouseCapture
            )
            .map_err(|_| ())
            .expect("failed to execute leave alternate screen, with unexpected error");

            // terminal.show_cursor(); // is it async?
        }
    }
}

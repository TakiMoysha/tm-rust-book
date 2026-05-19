pub mod app;
pub mod ui;

use std::{
    io::{self},
    path::absolute,
    time::{Duration, Instant},
};

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::{App, InputMode};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct ArgumentsParser {
    init_file: Option<String>,
}

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut out_pipe = io::stdout();
    execute!(out_pipe, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(out_pipe))?;

    // parse args
    let args = ArgumentsParser::parse();

    // init app
    let mut app = if let Some(init_file) = args.init_file {
        let init_file = absolute(&init_file)
            .context("Could not get absolute path")?
            .to_string_lossy()
            .to_string();
        App::with_file(&init_file)
    } else {
        App::new()
    };

    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor().context("Failed to show cursor")?;

    result.map_err(|error| anyhow::anyhow!(error))
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    while !app.should_quit {
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, app))?;

        // Handle events with timeout
        if event::poll(tick_rate.saturating_sub(last_tick.elapsed()))? {
            if let Event::Key(key) = event::read()? {
                // Only process key press, not release or repeat
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.input_mode {
                    InputMode::Normal => handle_normal_mode(app, key.code, key.modifiers),
                    InputMode::Insert => handle_insert_mode(app, key.code, key.modifiers),
                }
            }
        }

        // Tick for any background updates
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn handle_normal_mode(app: &mut App, code: KeyCode, _modifiers: KeyModifiers) {
    match code {
        KeyCode::Char('?') => app.notify("Your in NORMAL mode, press i to INSERT"),
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('i') | KeyCode::Char('I') => app.enter_insert_mode(),
        KeyCode::Char('r') | KeyCode::Char('R') => app.try_parse_file(),
        KeyCode::Down | KeyCode::Char('j') => app.select_next(),
        KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
        KeyCode::Char('g') => app.select_first(),
        KeyCode::Char('G') => app.select_last(),
        _ => {}
    }
}

fn handle_insert_mode(app: &mut App, code: KeyCode, _modifiers: KeyModifiers) {
    if app.show_suggestions {
        handle_suggestions_mode(app, code);
        return;
    }

    match code {
        KeyCode::Esc => app.enter_normal_mode(),
        KeyCode::Enter => app.enter_normal_mode(),
        KeyCode::Tab => app.open_suggestions(),
        KeyCode::Char(c) => app.insert_char(c),
        KeyCode::Backspace => app.backspace(),
        KeyCode::Delete => app.delete_char(),
        KeyCode::Left => app.move_cursor_left(),
        KeyCode::Right => app.move_cursor_right(),
        KeyCode::Home => app.move_cursor_start(),
        KeyCode::End => app.move_cursor_end(),
        _ => {}
    }
}

fn handle_suggestions_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => app.close_suggestions(),
        KeyCode::Enter => app.accept_suggestion(),
        KeyCode::Tab => app.select_next_suggestion(),
        KeyCode::Down => app.select_next_suggestion(),
        KeyCode::Up => app.select_prev_suggestion(),
        KeyCode::Right => app.suggestion_enter_dir(),
        KeyCode::Char(c) => app.suggestion_insert_char(c),
        KeyCode::Backspace => app.suggestion_backspace(),
        _ => {}
    }
}

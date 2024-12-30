use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{palette::material::YELLOW, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

#[derive(Debug, Default)]
pub struct CounterApp {
    counter: u8,
    exit: bool,
}

impl CounterApp {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Up => self.increment_counter(),
            KeyCode::Down => self.decrement_counlter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counlter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &CounterApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Down>".blue().bold(),
            " Increment ".into(),
            "<Up>".blue().bold(),
            " Quit ".into(),
            "<Esc>".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn should_correct_draw_tui() {
        let app = CounterApp::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
        app.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━━ Decrement <Down> Increment <Up> Quit <Esc>━━━┛",
        ]);

        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(14, 3, 6, 1), key_style);
        expected.set_style(Rect::new(31, 3, 4, 1), key_style);
        expected.set_style(Rect::new(41, 3, 5, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn should_correct_handle_events() -> io::Result<()> {
        let mut app = CounterApp::default();
        app.handle_key_event(KeyCode::Up.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Down.into());
        assert_eq!(app.counter, 0);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
        Ok(())
    }
}

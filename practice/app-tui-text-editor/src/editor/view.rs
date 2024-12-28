mod buffer;
mod line;
mod location;

use buffer::Buffer;
use location::Location;

use super::{
    editor_command::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};

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

#[derive(Debug)]
pub struct View {
    buffer: Buffer,
    size: Size,
    should_redraw: bool,
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            size: Terminal::size().unwrap(),
            should_redraw: true,
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.should_redraw = true;
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_sub(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_sub(1);
            offset_changed = true;
        }

        self.should_redraw = offset_changed;
    }

    pub fn render(&mut self) {
        if !self.should_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let vertical_center = height / 3;

        let top = self.scroll_offset.y;
        for current_row in 0..height {
            if let Some(line) = self.buffer.data.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::draw_line(current_row, &line.get_visible_graphemes(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::draw_line(current_row, &Self::build_welcome(width));
            } else {
                Self::draw_line(current_row, "~");
            }
        }
        self.should_redraw = true;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Idle | EditorCommand::Quit => {}
        }
    }
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = buffer::Buffer::load(file_name) {
            self.buffer = buffer;
            self.should_redraw = true;
        };
    }

    fn draw_line(at: usize, text: &str) {
        let res = Terminal::print_line(at, text);
        debug_assert!(res.is_ok(), "Failed to render line");
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

    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = self.size;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Left => {
                x = x.saturating_sub(1);
            }
            Direction::Right => {
                x = x.saturating_add(1);
            }
            Direction::PageUp => {
                y = 0;
            }
            Direction::PageDown => {
                y = height.saturating_sub(1);
            }
            Direction::Home => {
                x = 0;
            }
            Direction::End => {
                x = width.saturating_sub(1);
            }
        }
        self.location = Location { x, y };
        self.scroll_location_into_view();
    }
}

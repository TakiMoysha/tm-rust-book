use std::io::Error;
use std::time::{Duration, Instant};

use super::size::Size;
use super::{terminal::Terminal, ui_components::UIComponent};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Default)]
pub struct MessageBar {
    current_msg: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool,
}

impl MessageBar {
    pub fn update_msg(&mut self, msg: &str) {
        self.current_msg = Message {
            text: msg.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.set_redraw_flag(true);
    }
}

impl UIComponent for MessageBar {
    fn set_redraw_flag(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn is_should_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_msg.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _size: Size) {}

    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        if self.current_msg.is_expired() {
            self.cleared_after_expiry = true;
        }

        let msg = if self.current_msg.is_expired() {
            ""
        } else {
            &self.current_msg.text
        };

        Terminal::print_row(origin, msg)
    }
}

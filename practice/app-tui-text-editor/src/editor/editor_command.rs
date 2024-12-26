use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Idle,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(EditorCommand::Quit),
                (KeyCode::Up, _) => Ok(EditorCommand::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(EditorCommand::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(EditorCommand::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(EditorCommand::Move(Direction::Right)),
                (KeyCode::Home, _) => Ok(EditorCommand::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(EditorCommand::Move(Direction::End)),
                (KeyCode::PageUp, _) => Ok(EditorCommand::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(EditorCommand::Move(Direction::PageDown)),
                _ => Ok(EditorCommand::Idle),
            },

            Event::Resize(width_u16, height_u16) => {
                let (width, height) = (width_u16 as usize, height_u16 as usize);
                Ok(EditorCommand::Resize(Size { width, height }))
            }

            _ => Err(format!("Invalid event: {event:?}")),
        }
    }
}

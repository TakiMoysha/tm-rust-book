use std::io::Error;

use super::{debug_panic, size::Size};

/// A component that can be rendered to the terminal
pub trait UIComponent {
    /// Set the redraw flag, if the component needs to be redrawn
    fn set_redraw_flag(&mut self, value: bool);

    /// Check if the component needs to be redrawn
    fn is_should_redraw(&self) -> bool;

    /// Update the size of the component and marks as redraw-needed
    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_redraw_flag(true);
    }

    /// Set the new size, Calling from `resize`, needs to be implemented by each componetn.
    fn set_size(&mut self, size: Size);

    /// Draw the component if it's need of redrawing
    fn render(&mut self, origin_y: usize) {
        if self.is_should_redraw() {
            match self.draw(origin_y) {
                Ok(()) => self.set_redraw_flag(false),
                Err(err) => debug_panic!("Failed to render component: {err}"),
            }
        }
    }

    /// Method to actually draw the component, must be implemented by each component
    fn draw(&mut self, origin_y: usize) -> Result<(), Error>;
}

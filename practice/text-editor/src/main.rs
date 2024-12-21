#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

mod editor;

fn main() {
    editor::Editor::default().run();
}

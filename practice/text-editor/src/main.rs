#![warn(clippy::all, clippy::print_stdout)]

mod editor;

fn main() {
    editor::Editor::default().run();
}

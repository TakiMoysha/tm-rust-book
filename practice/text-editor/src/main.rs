#![warn(clippy::all, clippy::print_stdout)]

mod editor;

mod panic_test;

fn main() {
    editor::Editor::default().run();
}

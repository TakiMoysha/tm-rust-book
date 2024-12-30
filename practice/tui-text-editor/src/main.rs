mod macros;
mod editor;

fn main() {
    editor::Editor::new().unwrap().run();
}

mod editor;
mod hooks;

fn main() {
    hooks::panic_hook();
    editor::Editor::default().run();
}

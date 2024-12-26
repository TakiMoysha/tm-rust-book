use crossterm::terminal::disable_raw_mode;

pub fn panic_hook() {
    let current_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        disable_raw_mode();
        current_hook(info);
    }))
}

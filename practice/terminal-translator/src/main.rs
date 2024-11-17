mod counter;

use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = counter::CounterApp::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

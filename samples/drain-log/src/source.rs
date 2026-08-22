use std::io::BufRead;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

/// Messages flowing from a reader thread to the main loop.
pub enum Msg {
    /// A single log line.
    Line(String),
    /// The input stream hit EOF.
    Eof,
    /// Ctrl-C was pressed; stop and produce output.
    Stop,
}

/// Where the log lines come from.
pub enum Input {
    File(PathBuf),
    Stdin,
}

/// Spawn a reader thread that streams lines from the input, one `Msg::Line` at
/// a time. In live mode (e.g. `tail -f | drain-log`) it blocks until EOF, so
/// the process is stopped via Ctrl-C (`Msg::Stop`); with a finite stream it
/// sends `Msg::Eof` and returns.
pub fn spawn_reader(input: Input, tx: Sender<Msg>) -> JoinHandle<()> {
    match input {
        Input::File(path) => std::thread::spawn(move || {
            if let Ok(file) = std::fs::File::open(&path) {
                for line in std::io::BufReader::new(file).lines() {
                    if !forward(line, &tx) {
                        return;
                    }
                }
            }
            let _ = tx.send(Msg::Eof);
        }),
        Input::Stdin => std::thread::spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                if !forward(line, &tx) {
                    return;
                }
            }
            let _ = tx.send(Msg::Eof);
        }),
    }
}

fn forward(line: Result<String, std::io::Error>, tx: &Sender<Msg>) -> bool {
    match line {
        Ok(line) => tx.send(Msg::Line(line)).is_ok(),
        // Invalid UTF-8: skip the line but keep reading.
        Err(_) => true,
    }
}

/// Install a Ctrl-C handler that reports `Msg::Stop` on `tx`.
pub fn install_ctrlc(tx: Sender<Msg>) -> anyhow::Result<()> {
    ctrlc::set_handler(move || {
        let _ = tx.send(Msg::Stop);
    })?;
    Ok(())
}

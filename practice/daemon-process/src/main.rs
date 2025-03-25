use std::fs::File;

use daemonize::Daemonize;

fn main() {
    let stdout = File::create("/tmp/daemon.log").unwrap();
    let stderr = File::create("/tmp/daemon.log").unwrap();

    let daemon = Daemonize::new()
        .pid_file("/tmp/rust-demo-daemon.pid")
        .chown_pid_file(true)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemon.start() {
        Ok(_) => println!("Daemon started"),
        Err(e) => eprintln!("Failed to start daemon: {}", e),
    }
}

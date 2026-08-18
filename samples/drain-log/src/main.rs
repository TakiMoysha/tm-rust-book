use std::{io::Read, path::PathBuf};

use clap::{Command, Parser, arg};

#[derive(Parser)]
#[command(name = "drain-log")]
#[command(version = "0.1.0")]
#[command(about = "TUI+MCP based on drainlog, for log analysis.", long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
    #[arg(long, default_value_t = 0.5)]
    sim_threshold: f64,
    #[arg(long, default_value_t = 4)]
    depth: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let sim_threshold = args.sim_threshold;
    let depth = args.depth;

    println!("DEBUG: sim_threshold: {}, depth: {}", sim_threshold, depth);

    let lines = match &args.file {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s)?;
            s
        }
    };

    let lines: Vec<String> = lines.lines().map(str::to_owned).collect();

    let miner = logdrain::Miner::builder()
        .sim_threshold(sim_threshold)
        .depth(depth)
        .parametrize_numeric_tokens(true)
        .masks([logdrain::builtin_masks::uuid(), logdrain::builtin_masks::ipv4()])
        .build()?;

    // for line in &input_file {
    //     let res = miner.add(&[line.as_str()]);
    // }
    // let custers = miner.clusters();

    Ok(())
}

#[derive(Debug)]
struct TuiState {
    active_tab: usize,
    raw: Vec<String>,
    clusters: Vec<logdrain::Cluster>,
    scroll: usize,

    selected_cluster: Option<logdrain::Cluster>,
    drain_tab_state: DrainTabState,
}

#[derive(Debug)]
struct DrainTabState {
    scroll: usize,
}

/// .
///
/// mapping dispatch:
///     - tab/{r,l}_arrow - tabs switching
///     - j/k & {u,d}_arrow - scroll up/down
///     - ? - help/options
///     - q - quit
///
fn render(state: &TuiState) {}

use std::sync::mpsc;

use clap::Parser;
use drain_log::cli::Cli;
use drain_log::miner::{self, MinerConfig, Stats};
use drain_log::source::{self, Input, Msg};
use logdrain::Miner;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let config = MinerConfig::from(&args);
    let miner = miner::build_miner(&config)?;
    let input = match &args.file {
        Some(path) if path.as_os_str() != "-" => Input::File(path.clone()),
        _ => Input::Stdin,
    };

    if args.dump {
        let (miner, stats) = consume(input, miner)?;
        dump(&miner, &stats)?;
        return Ok(());
    }

    drain_log::ui::run(input, miner)
}

/// Non-interactive debug path: ingest everything, then print clusters.
fn consume(input: Input, miner: Miner) -> anyhow::Result<(Miner, Stats)> {
    let (tx, rx) = mpsc::channel::<Msg>();
    source::install_ctrlc(tx.clone())?;
    let _reader = source::spawn_reader(input, tx.clone());
    drop(tx);

    let mut stats = Stats::default();
    while let Ok(msg) = rx.recv() {
        match msg {
            Msg::Line(line) => miner::ingest(&miner, &mut stats, &line),
            Msg::Eof | Msg::Stop => break,
        }
    }
    Ok((miner, stats))
}

fn dump(miner: &Miner, stats: &Stats) -> anyhow::Result<()> {
    println!("lines: {}  clusters: {}", stats.lines, miner.len());
    for cluster in drain_log::ui::sorted_clusters(&miner.clusters()) {
        println!("{}\t{}\t{}", cluster.id(), cluster.size(), cluster.template());
    }
    Ok(())
}

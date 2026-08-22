use std::io::BufRead;
use std::path::Path;

use drain_log::miner::{MinerConfig, Stats, build_miner, ingest};
use logdrain::Miner;

fn mine(path: &Path) -> (Miner, Stats) {
    let config = MinerConfig {
        path_delimiters: vec!['/'],
        ..MinerConfig::default()
    };
    let miner = build_miner(&config).unwrap();
    let mut stats = Stats::default();
    let file = std::fs::File::open(path).unwrap();
    for line in std::io::BufReader::new(file).lines() {
        ingest(&miner, &mut stats, &line.unwrap());
    }
    (miner, stats)
}

fn cluster_by_substring(miner: &Miner, needle: &str) -> Option<logdrain::Cluster> {
    miner.clusters().into_iter().find(|c| c.template().contains(needle))
}

#[test]
fn mines_test_log_into_clusters() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test.log");
    let (miner, stats) = mine(&path);

    assert_eq!(stats.lines, 115, "non-blank lines in test.log");
    assert_eq!(stats.with_timestamp, 115, "all lines carry RFC3339 prefixes");
    assert!(miner.len() > 1, "expected several clusters, got {}", miner.len());
}

#[test]
fn identical_lines_share_a_cluster() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test.log");
    let (miner, _) = mine(&path);

    let c = cluster_by_substring(&miner, "Flow Worker started in new thread").unwrap();
    assert_eq!(c.size(), 6, "six identical flow-worker lines");
    assert!(c.template().starts_with("<ts>"), "timestamp masked: {}", c.template());
}

#[test]
fn varying_durations_generalize_to_wildcard() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test.log");
    let (miner, _) = mine(&path);

    let c = cluster_by_substring(&miner, "Recovering raft logs takes").unwrap();
    assert_eq!(c.size(), 2, "lines 3 and 10 share a template");
    assert!(
        c.template().ends_with("Recovering raft logs takes <*>"),
        "template: {}",
        c.template()
    );
}

#[test]
fn region_ids_are_masked() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test.log");
    let (miner, _) = mine(&path);

    let c = cluster_by_substring(&miner, "Try to open region").unwrap();
    assert!(c.template().contains("<region>"), "template: {}", c.template());
}

#[test]
fn http_error_lines_group_by_endpoint() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test.log");
    let (miner, _) = mine(&path);

    let hits: u64 = miner
        .clusters()
        .iter()
        .filter(|c| c.template().contains("HTTP error response 400"))
        .map(|c| c.size())
        .sum();
    assert_eq!(hits, 7, "seven 400 responses across endpoint clusters");
    let c = cluster_by_substring(&miner, "HTTP error response 400").unwrap();
    assert!(c.template().contains("<ip>"), "client ip:port masked: {}", c.template());
}

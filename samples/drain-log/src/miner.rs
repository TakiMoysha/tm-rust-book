use anyhow::Context;
use logdrain::{Mask, Miner, UpdateType, builtin_masks};

use crate::cli::Cli;
use crate::time;

/// Resolved logdrain builder configuration (independent of clap for testability).
pub struct MinerConfig {
    pub sim_threshold: f64,
    pub depth: usize,
    pub max_clusters_per_leaf: usize,
    pub first_line_only: bool,
    pub path_delimiters: Vec<char>,
    pub custom_masks: Vec<(String, String)>,
}

impl Default for MinerConfig {
    fn default() -> Self {
        MinerConfig {
            sim_threshold: 0.5,
            depth: 4,
            max_clusters_per_leaf: 100,
            first_line_only: false,
            path_delimiters: Vec::new(),
            custom_masks: Vec::new(),
        }
    }
}

impl From<&Cli> for MinerConfig {
    fn from(c: &Cli) -> Self {
        MinerConfig {
            sim_threshold: c.sim_threshold,
            depth: c.depth,
            max_clusters_per_leaf: c.max_clusters_per_leaf,
            first_line_only: c.first_line_only,
            path_delimiters: c.path_delimiters.clone().unwrap_or_default(),
            custom_masks: c.masks.clone(),
        }
    }
}

/// Build a `logdrain::Miner` from the resolved config: numeric parametrization,
/// a default mask set (uuid/ipv4/hex32/region-id), optional path splitting and
/// user-supplied regex masks.
pub fn build_miner(config: &MinerConfig) -> anyhow::Result<Miner> {
    let mut builder = Miner::builder()
        .sim_threshold(config.sim_threshold)
        .depth(config.depth)
        .max_clusters_per_leaf(config.max_clusters_per_leaf)
        .parametrize_numeric_tokens(true)
        .first_line_only(config.first_line_only);
    if !config.path_delimiters.is_empty() {
        builder = builder.path_delimiters(&config.path_delimiters);
    }
    let mut masks = vec![
        timestamp_mask(),
        builtin_masks::uuid(),
        ip_port_mask(),
        builtin_masks::ipv4(),
        builtin_masks::hex32(),
        region_mask(),
    ];
    for (pattern, placeholder) in &config.custom_masks {
        masks
            .push(Mask::new(pattern, placeholder).with_context(|| format!("invalid --mask: {pattern}={placeholder}"))?);
    }
    builder = builder.masks(masks);
    builder.build().map_err(Into::into)
}

/// RFC3339 timestamp prefix -> `<ts>`. Without this, every unique timestamp is a
/// distinct token, so depth-limited prefix descent sends each line to its own
/// leaf and nothing ever clusters.
fn timestamp_mask() -> Mask {
    Mask::new(
        r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})",
        "<ts>",
    )
    .expect("timestamp mask is valid")
}

/// High-cardinality region ids like `4398046511104(1024, 0)` -> `<region>`.
fn region_mask() -> Mask {
    Mask::new(r"\b\d{1,20}\(\d{1,5}, \d{1,5}\)", "<region>").expect("region-id mask is valid")
}

/// `ip:port` pairs -> `<ip>` (applied before the bare ipv4 mask so the port
/// cannot get generalized into a separate `<*>`).
fn ip_port_mask() -> Mask {
    Mask::new(r"\b(?:\d{1,3}\.){3}\d{1,3}:\d+\b", "<ip>").expect("ip:port mask is valid")
}

/// Per-run counters.
#[derive(Debug, Default)]
pub struct Stats {
    pub lines: u64,
    pub with_timestamp: u64,
    pub blank: u64,
    pub invalid_utf8: u64,
    pub created: u64,
    pub template_changed: u64,
    pub matched: u64,
}

/// Feed one log line into the miner. Parses an RFC3339 prefix when present and
/// uses `add_at` so clusters carry real event-time windows; otherwise falls back
/// to `add`.
pub fn ingest(miner: &Miner, stats: &mut Stats, line: &str) {
    if line.trim().is_empty() {
        stats.blank += 1;
        return;
    }
    stats.lines += 1;
    let result = match time::parse_prefix_ms(line) {
        Some(ms) => {
            stats.with_timestamp += 1;
            miner.add_at(line, ms)
        }
        None => miner.add(line),
    };
    match result.update {
        UpdateType::Created => stats.created += 1,
        UpdateType::TemplateChanged => stats.template_changed += 1,
        UpdateType::None => stats.matched += 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_succeeds_with_defaults() {
        let miner = build_miner(&MinerConfig::default()).unwrap();
        assert_eq!(miner.len(), 0);
    }

    #[test]
    fn ingest_parses_event_time_and_counts() {
        let miner = build_miner(&MinerConfig::default()).unwrap();
        let mut stats = Stats::default();
        ingest(&miner, &mut stats, "2026-08-13T17:41:44.619908Z a b c");
        ingest(&miner, &mut stats, "2026-08-13T17:41:44.620000Z a b c");
        ingest(&miner, &mut stats, "   ");
        ingest(&miner, &mut stats, "no timestamp here");
        assert_eq!(stats.lines, 3);
        assert_eq!(stats.with_timestamp, 2);
        assert_eq!(stats.blank, 1);
        assert_eq!(stats.created, 2);
        assert_eq!(stats.matched, 1);
    }

    #[test]
    fn region_mask_collapses_high_cardinality_ids() {
        let config = MinerConfig {
            path_delimiters: vec!['/'],
            ..MinerConfig::default()
        };
        let miner = build_miner(&config).unwrap();
        let mut stats = Stats::default();
        ingest(
            &miner,
            &mut stats,
            "2026-08-13T17:41:44Z mito2::worker::handle_open: Try to open region 4398046511104(1024, 0), worker: 4",
        );
        ingest(
            &miner,
            &mut stats,
            "2026-08-13T17:41:44Z mito2::worker::handle_open: Try to open region 4440996184064(1034, 0), worker: 2",
        );
        assert_eq!(miner.len(), 1);
        let cluster = miner.cluster(1).unwrap();
        let t = cluster.template();
        assert!(t.contains("<region>"), "template: {t}");
        assert!(t.contains("<*>"), "template: {t}");
    }
}

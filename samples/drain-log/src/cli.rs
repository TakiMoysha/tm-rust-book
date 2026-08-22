use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "drain-log", version, about = "Cluster log lines into templates using logdrain (Drain), with a TUI.", long_about = None)]
pub struct Cli {
    /// Input file. Reads from stdin (live) when omitted or '-'.
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Similarity threshold in [0.0, 1.0] for joining an existing cluster.
    #[arg(long, default_value_t = 0.5)]
    pub sim_threshold: f64,

    /// Prefix-tree depth (>= 2).
    #[arg(long, default_value_t = 4)]
    pub depth: usize,

    /// Max clusters kept per prefix leaf before LRU eviction.
    #[arg(long, default_value_t = 100)]
    pub max_clusters_per_leaf: usize,

    /// Cluster on the first line only (for multi-line records); the rest is kept
    /// as the cluster suffix.
    #[arg(long)]
    pub first_line_only: bool,

    /// Chars that split a token into path sub-tokens, e.g. '/' or '/,.'.
    #[arg(long, value_delimiter = ',')]
    pub path_delimiters: Option<Vec<char>>,

    /// Extra regex mask PATTERN=PLACEHOLDER, repeatable.
    #[arg(long = "mask", value_parser = parse_mask)]
    pub masks: Vec<(String, String)>,

    /// Non-interactive debug output: ingest everything, print clusters, exit.
    #[arg(long, hide = true)]
    pub dump: bool,
}

fn parse_mask(s: &str) -> Result<(String, String), String> {
    let (pattern, placeholder) = s
        .split_once('=')
        .ok_or_else(|| format!("expected PATTERN=PLACEHOLDER, got '{s}'"))?;
    if pattern.is_empty() || placeholder.is_empty() {
        return Err(format!("expected PATTERN=PLACEHOLDER, got '{s}'"));
    }
    Ok((pattern.to_string(), placeholder.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mask_splits_on_first_equals() {
        assert_eq!(
            parse_mask(r"\bport \d+\b=<port>").unwrap(),
            (r"\bport \d+\b".to_string(), "<port>".to_string())
        );
    }

    #[test]
    fn parse_mask_rejects_bad_input() {
        assert!(parse_mask("no-equals").is_err());
        assert!(parse_mask("=empty").is_err());
        assert!(parse_mask("empty=").is_err());
    }
}

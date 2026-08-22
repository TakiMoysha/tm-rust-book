use jiff::Timestamp;

/// Parse an RFC3339 timestamp prefix from the start of a log line, returning
/// unix-milliseconds. Returns `None` when the line has no usable prefix.
pub fn parse_prefix_ms(line: &str) -> Option<u64> {
    let token = line.split_whitespace().next()?;
    let ts: Timestamp = token.parse().ok()?;
    let ms = ts.as_millisecond();
    (ms > 0).then_some(ms as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_rfc3339() {
        assert_eq!(
            parse_prefix_ms("2026-08-13T17:41:44.619908Z  INFO x"),
            Some(1786642904619)
        );
    }

    #[test]
    fn parses_second_precision() {
        assert_eq!(parse_prefix_ms("2026-08-13T17:41:44Z INFO x"), Some(1786642904000));
    }

    #[test]
    fn parses_numeric_offset() {
        assert_eq!(
            parse_prefix_ms("2026-08-13T19:41:44.619908+02:00  WARN x"),
            Some(1786642904619)
        );
    }

    #[test]
    fn rejects_missing_or_non_timestamp_prefix() {
        assert_eq!(parse_prefix_ms(""), None);
        assert_eq!(parse_prefix_ms("no timestamp here"), None);
        assert_eq!(parse_prefix_ms("42 hello"), None);
        assert_eq!(parse_prefix_ms("INFO plain log line"), None);
    }
}

# LR-0001 — Prior knowledge & mission established

User disclosed solid Rust language fundamentals (ownership, lifetimes not a teaching target); frontier is ecosystem crates — tokio async runtime, surrealdb SDK 3.x, ratatui 0.30. Mission set: career/portfolio-driven, taught exclusively through building the `tui-hybrid-search` DB-explorer TUI (browse tables/records, run SurrealQL).

**Implications:** Lessons skip language pedagogy entirely; difficulty should come from architecture and crate APIs, not syntax. Project state at start: does not compile (missing tokio dep, undefined `Database`/`is_schema_exists`, broken `infra/mod.rs`). Teaching sequence starts with the data-layer vertical slice.

# Mission: Rust — DB-explorer TUI for SurrealDB

## Why
Become employable as a Rust developer by building real, portfolio-grade software. The vehicle is `tui-hybrid-search`: a terminal UI that connects to SurrealDB, browses tables and records, and runs SurrealQL — a mini database client built from scratch.

## Success looks like
- A running TUI app: connect to SurrealDB, list tables/records, run arbitrary SurrealQL from an input line, view results in panes
- Code that shows idiomatic async Rust (tokio), a clean split between data layer (surrealdb SDK) and UI layer (ratatui), real error handling
- A repo the user can walk an interviewer through, explaining every architectural choice

## Constraints
- Rust language fundamentals are already solid — never re-teach ownership/borrowing basics
- All learning happens inside this repo, on this project
- Short sessions, one tangible win per lesson

## Out of scope
- General Rust language pedagogy (The Book chapters)
- Vector / full-text hybrid search features until the explorer core works
- Other sample projects in tm-rust-book

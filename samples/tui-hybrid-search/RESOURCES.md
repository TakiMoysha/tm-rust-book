# Rust TUI + SurrealDB Resources

## Knowledge

- [SurrealDB docs — Rust embedding](https://surrealdb.com/docs/surrealdb/reference/rust/embedding)
  Official SDK install & embedded-engine guide (`Mem`, `RocksDb`, feature flags like `kv-mem`). Use for: connecting without a server, Cargo features.
- [SurrealDB docs — Rust SDK methods](https://surrealdb.com/docs/surrealdb/reference/rust/methods)
  Reference for `create`, `select`, `query`, `update`, `delete` from Rust. Use for: every data-layer lesson.
- [SurrealDB docs — SurrealQL](https://surrealdb.com/docs/surrealql)
  The query language itself: statements, functions, schema definitions. Use for: anything the explorer must execute/display.
- [ratatui docs (0.30)](https://docs.rs/ratatui/0.30.0/ratatui/)
  Crate reference incl. `init`/`restore`/`run` lifecycle and widgets. Use for: all UI lessons; version matches Cargo.toml.
- [The Ratatui Book](https://ratatui.rs/concepts/)
  Concept guides: app architecture, event handling, rendering model. Use for: design patterns before API details.
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
  Official async-runtime tutorial. Use for: the async frontier (runtimes, tasks, channels) when UI and DB need to run concurrently.

## Wisdom (Communities)

- [r/rust](https://www.reddit.com/r/rust/)
  Largest Rust community; weekly "hey rustaceans" questions thread is ideal for beginner-intermediate code review asks.
- [SurrealDB Discord](https://discord.gg/surrealdb) / [GitHub Discussions](https://github.com/surrealdb/surrealdb/discussions)
  Direct access to SDK maintainers; best place for version-specific SDK behavior that docs don't cover.
- [Ratatui Discord](https://discord.gg/aK6ADk2nhv)
  Linked from the ratatui book; widget/layout critique from people who build TUIs daily.

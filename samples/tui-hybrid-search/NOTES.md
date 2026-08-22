# Teaching notes

## User profile
- Career-focused (employability is the driver); building a portfolio-grade DB-explorer TUI
- Self-reports Rust language fundamentals are solid — do NOT teach ownership/lifetimes basics
- Frontier: crates & ecosystem — tokio async runtime, surrealdb SDK, ratatui architecture
- Wants lessons centered on THIS project only (`samples/tui-hybrid-search`)
- Target app shape: mini DB client (browse tables/records, run SurrealQL, view results) — NOT hybrid search yet

## Preferences
- Concise communication style; CLI context
- Started the project data-layer-first (`src/infra/mod.rs` has a half-written static `LazyLock<Surreal<Client>>` DB handle) — meet them there

## Working notes
- Session 1 done: 0001 complete (user's main.rs compiles & runs; added SurrealValue derive unprompted). tokio pinned with `features=["full"]` — fine, not worth fighting
- surrealdb 3.2.3 pinned: local engines need feature flags (`kv-mem` for in-memory); `create()` = 1 record/Option, `insert()` = many/Vec (cheat sheet corrected)
- Lesson plan sketch: ~~0001 connect+CRUD slice~~ ✅ → 0002 schema/DEFINE + static handle refactor → 0003 ratatui hello-world loop → 0004 layout/table widget over real data → 0005 input line + SurrealQL passthrough → 0006 error UX + tracing

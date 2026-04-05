# CLAUDE.md

## Project

`ratatui-themekit` — semantic theme system for ratatui TUI applications.

## Commands

```bash
cargo build              # Build
cargo test               # Run tests
cargo clippy             # Lint
cargo fmt                # Format
cargo doc --open         # Generate and open docs
```

## Architecture

- `src/lib.rs` — Theme trait, resolve helpers, NO_COLOR support
- `src/themes.rs` — 7 built-in theme implementations

## Rules

- ALL colors go through Theme trait slots — zero hardcoded `Color::Rgb(...)` in consuming code
- Theme trait has 15 required methods + 5+ derived defaults
- `Send + Sync` required on Theme implementations
- NO_COLOR must always be respected
- Every public item must have doc comments
- Clippy pedantic enabled — zero warnings allowed

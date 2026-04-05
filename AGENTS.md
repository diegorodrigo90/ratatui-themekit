# AGENTS.md

Operational contract for AI coding assistants working on this repository.

## Golden Rules

1. ALL colors go through `Theme` trait slots — zero hardcoded `Color::Rgb(...)` or `Color::Red` in render code
2. Every public item has doc comments with examples
3. `Send + Sync` required on all Theme implementations
4. `NO_COLOR` must always be respected
5. Every built-in theme has tests verifying distinct status/diff colors
6. Conventional commits: `type(scope): description`

## Quality Gates (MANDATORY before commit)

```bash
cargo fmt --check            # Zero formatting issues
cargo clippy --all-targets   # Zero warnings (pedantic enabled)
cargo test                   # All tests pass
cargo doc --no-deps          # No doc warnings
cargo deny check             # License + advisory + source check
```

## Architecture

```
src/
  lib.rs       — Theme trait, resolve helpers, NO_COLOR, tests
  themes.rs    — Built-in theme implementations (9 themes)
  custom.rs    — CustomTheme with serde for user config files
```

## Adding a New Theme

1. Add struct in `src/themes.rs` implementing `Theme` trait
2. Register in `resolve_theme()` match arms
3. Add to `builtin_themes()` and `available_theme_ids()`
4. Add tests: distinct status colors, distinct diff colors
5. Update README theme table
6. Update CHANGELOG

## Design Principles

- **Semantic over literal**: slots describe meaning (`success`), not appearance (`green`)
- **Derived defaults**: 15 required methods → 20+ total (derived are overridable)
- **Zero opinion on layout**: only colors, never widget structure
- **Trait-based extensibility**: any app can implement custom themes
- **NO_COLOR native**: accessibility built in, not bolted on

## Code Standards

- Rust 2024 edition
- Clippy pedantic enabled (`clippy::pedantic` at warn level)
- All public items documented with `///` doc comments
- Builder functions use `impl Into<String>` for ergonomics
- Tests for every public function and every theme
- `#[must_use]` on all pure functions

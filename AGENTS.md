# AGENTS.md

Operational contract for AI coding assistants working on this repository.

## Golden Rules

1. ALL colors go through `Theme` trait slots — zero hardcoded `Color::Rgb(...)` or `Color::Red`
2. Every public item has doc comments
3. `Send + Sync` required on all Theme implementations
4. `NO_COLOR` must always be respected
5. Every built-in theme has tests verifying distinct status/diff colors
6. Conventional commits: `type(scope): description`
7. Files max 300 lines, single responsibility
8. `interactive: true` in lefthook for all cargo commands

## Quality Gates (MANDATORY before commit)

```bash
./scripts/validate.sh       # Full validation (or manually below)
cargo fmt --check            # Zero formatting issues
cargo clippy --all-targets   # Zero warnings (pedantic enabled)
cargo test --all-features    # All tests pass (including serde)
cargo doc --no-deps          # No doc warnings
cargo deny check             # License + advisory + source check
```

## Architecture

```
src/
├── lib.rs              ← re-exports + integration tests
├── theme_trait.rs      ← Theme trait (15 required + 12 derived)
├── resolve.rs          ← resolve_theme(), builtin_themes(), no_color_active()
├── builders/
│   ├── mod.rs          ← ThemeExt trait + style_* helpers
│   ├── span.rs         ← ThemedSpan builder
│   ├── bar.rs          ← ThemedBar builder
│   ├── block.rs        ← ThemedBlock (themed Block widget)
│   ├── line.rs         ← ThemedLine (multi-span line compositor)
│   ├── status_line.rs  ← ThemedStatusLine (key-value status bar)
│   └── styles.rs       ← TableStyles, ListStyles, TabStyles, GaugeStyles, StateStyles
└── themes/
    ├── mod.rs           ← ThemeData struct + BUILTIN_THEMES registry
    ├── <theme_name>.rs  ← one file per theme (25 lines each)
    └── custom.rs        ← CustomTheme (serde runtime themes)
```

## Adding a New Theme

1. Create `src/themes/<name>.rs` with `pub const NAME: ThemeData = ThemeData { ... };`
2. In `src/themes/mod.rs`: add `mod <name>;`, `pub use <name>::NAME;`, add to `BUILTIN_THEMES`
3. In `src/lib.rs` aliases: add `pub const PascalName: ThemeData = NAME;`
4. Tests auto-validate all themes in the registry
5. Update README theme table

## Design Principles

- **Semantic over literal**: slots describe meaning (`success`), not appearance (`green`)
- **Data-driven themes**: `ThemeData` struct, not code repetition
- **Single registry**: `BUILTIN_THEMES` is the single source of truth
- **Derived defaults**: 15 required → 25+ total (derived are overridable)
- **Zero opinion on layout**: only colors, never widget structure
- **Trait-based extensibility**: any app can implement custom themes
- **`NO_COLOR` native**: accessibility built in, not bolted on

## Showcase, Screenshots & Docs (GOLDEN RULE — NEVER skip)

**EVERY commit that changes builders, themes, style bundles, or ThemeExt MUST update:**

1. **`examples/showcase.rs`** — add demo section for new feature (ALL builders must be visible)
2. **PNGs + GIF** — `./scripts/generate-screenshots.sh` (requires `vhs`)
3. **`README.md`** — update Builders section, theme table, examples
4. **`examples/README.md`** — theme gallery with all PNGs
5. **`CLAUDE.md` + `AGENTS.md`** — architecture section if file structure changed
6. **`CONTRIBUTING.md`** — release checklist if process changed

Commands:
- `cargo run --example showcase` — interactive TUI demo with all builders
- `./scripts/generate-screenshots.sh` — generates all PNGs + GIF
- `showcase.rs` reads version and theme count dynamically via `env!("CARGO_PKG_VERSION")`
- `assets/` is excluded from crate publish (Cargo.toml `exclude`)

**A feature without updated showcase + docs + screenshots is NOT done.**

## Code Standards

- Rust 2024 edition, clippy pedantic
- All public items documented with `///`
- `#[must_use]` on all pure functions
- Tests colocated in each file
- No git co-author footers

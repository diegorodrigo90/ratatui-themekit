# CLAUDE.md

## Project

`ratatui-themekit` — semantic theme system for [ratatui](https://ratatui.rs) TUI applications.

## Commands

```bash
cargo build                 # Build
cargo test --all-features   # Run all tests (including serde)
cargo clippy --all-targets  # Lint (pedantic)
cargo fmt                   # Format
cargo doc --open            # Generate and view docs
cargo run --example showcase  # Run showcase example
./scripts/validate.sh       # Full validation (fmt + clippy + test + deny + rustdoc)
```

## Architecture

```
src/
├── lib.rs              ← re-exports + integration tests
├── theme_trait.rs      ← Theme trait (15 required + 12 derived methods)
├── resolve.rs          ← resolve_theme(), builtin_themes(), no_color_active()
├── builders/
│   ├── mod.rs          ← ThemeExt trait + style helpers
│   ├── span.rs         ← ThemedSpan (chainable span builder)
│   ├── bar.rs          ← ThemedBar (progress bar builder)
│   ├── block.rs        ← ThemedBlock (themed Block widget builder)
│   ├── line.rs         ← ThemedLine (multi-span line compositor)
│   ├── status_line.rs  ← ThemedStatusLine (key-value status bar)
│   └── styles.rs       ← TableStyles, ListStyles, TabStyles, GaugeStyles, StateStyles, zebra_rows
└── themes/
    ├── mod.rs           ← ThemeData struct + BUILTIN_THEMES registry
    ├── catppuccin_mocha.rs  ← 1 theme = 1 file = 25 lines
    ├── dracula.rs
    ├── ... (11 themes total)
    └── custom.rs        ← CustomTheme (serde, runtime user themes)
```

## Rules

- ALL colors through Theme trait — zero hardcoded `Color::*`
- Theme trait: 15 required + 12 derived methods
- ThemeExt: span, line, block, status_line, table/list/tab/gauge/state style bundles
- `Send + Sync` required on implementations
- `NO_COLOR` always respected
- Every public item has doc comments
- Clippy pedantic — zero warnings
- Conventional commits
- Files max 450 lines, single responsibility
- `interactive: true` in lefthook for cargo commands (prevents jobserver deadlock)

## Adding a New Theme

1. Create `src/themes/my_theme.rs` with a `pub const MY_THEME: ThemeData`
2. Add `mod my_theme;` and `pub use my_theme::MY_THEME;` in `src/themes/mod.rs`
3. Add `MY_THEME` to the `BUILTIN_THEMES` array in `src/themes/mod.rs`
4. Add PascalCase alias in `src/lib.rs` aliases module
5. Tests auto-validate (every theme in BUILTIN_THEMES is checked)

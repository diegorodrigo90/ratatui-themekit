# CLAUDE.md

## Project

`ratatui-themekit` — semantic theme system for [ratatui](https://ratatui.rs) TUI applications.

Part of the [Ralph Engine](https://ralphengine.com) ecosystem, extracted as a standalone crate for the ratatui community.

## Commands

```bash
cargo build                 # Build
cargo test                  # Run all tests
cargo clippy --all-targets  # Lint (pedantic)
cargo fmt                   # Format
cargo doc --open            # Generate and view docs
cargo deny check            # License + advisory check
```

## Architecture

```
src/
  lib.rs       — Theme trait (20 slots), resolve helpers, NO_COLOR
  themes.rs    — 9 built-in themes (Catppuccin, Dracula, Nord, ...)
  custom.rs    — CustomTheme with serde for user configs
```

## Rules

- ALL colors through Theme trait — zero hardcoded `Color::*` in consuming code
- Theme trait: 15 required + 5+ derived methods
- `Send + Sync` required on implementations
- `NO_COLOR` always respected
- Every public item has doc comments
- Clippy pedantic — zero warnings
- Conventional commits

## Key Design Decisions

- **Trait over struct**: `Theme` is a trait, not a struct — apps can implement custom themes
- **Derived defaults**: `block_pass()` defaults to `success()` — override only when needed
- **TailwindDark**: uses `ratatui::style::palette::tailwind` constants directly
- **TerminalNative**: uses ANSI named colors — respects user's terminal theme
- **CustomTheme**: serde-powered, all fields optional with sensible defaults

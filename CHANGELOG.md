# Changelog


### CI

- Automated release pipeline — release-plz creates tag, publish.yml publishes

### Fixed

- Clippy pedantic in showcase (truncation, clone, range) + release-plz git credentials
- Remove unused Style import in test (CI -Dwarnings)
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).
# Changelog


### Added

- Add background() derived slot, MSRV 1.86.0, CI MSRV check

### CI

- Add harden-runner to all jobs, fix audit-check SHA
- Fix deny + audit failures, add release checklist
- Fix audit permissions, ignore upstream time advisory
- Consolidate jobs with rust-cache, remove release-plz
- Restore release-plz with correct credentials, add rust-cache
- Use RELEASE_TOKEN PAT for release-plz push
- Fix release-plz — pass token to checkout, use latest action
- Fix release-plz action SHA — use release-plz/action main
- Add release-assets workflow for doc updates on release PRs
# Changelog


### Added

- Add InputStyles, ScrollbarStyles, NotificationStyles bundles

## [0.1.0] - 2026-04-05

### Added

- `Theme` trait with 20 semantic color slots (15 required + 5 derived)
- 9 built-in themes: Catppuccin Mocha, Dracula, Nord, Gruvbox Dark, One Dark, Solarized Dark, Tailwind Dark, Terminal Native, No Color
- `TailwindDark` theme using ratatui's built-in `palette::tailwind` constants
- `TerminalNative` theme using ANSI named colors (respects terminal theme)
- `NoColor` theme for `NO_COLOR` environment compliance
- `CustomTheme` struct with serde support for user-defined themes from config files
- `resolve_theme(id)` — resolve by config ID with sensible fallback
- `builtin_themes()` — list all built-in theme instances
- `available_theme_ids()` — list all built-in theme IDs
- `no_color_active()` — check `NO_COLOR` environment variable
- CI with check, fmt, clippy, test, doc jobs (GitHub Actions, SHA-pinned)
- Full documentation with examples

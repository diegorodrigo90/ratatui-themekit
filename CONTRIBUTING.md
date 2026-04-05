# Contributing

Thank you for your interest in contributing to ratatui-themekit!

## Getting Started

```bash
git clone https://github.com/diegorodrigo90/ratatui-themekit.git
cd ratatui-themekit
cargo build
cargo test
```

## Adding a New Theme

1. Add a struct in `src/themes.rs`
2. Implement the `Theme` trait (15 required methods)
3. Register in `resolve_theme()` and `builtin_themes()` in `src/lib.rs`
4. Add to the README table
5. Add tests

## Quality Gates

All must pass before submitting a PR:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

## Code Style

- Rust 2024 edition
- Clippy pedantic enabled
- All public items must have doc comments
- Use semantic color slot names, never raw RGB in docs/examples

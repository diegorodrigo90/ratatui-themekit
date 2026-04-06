# Contributing

## Getting Started

```bash
git clone https://github.com/diegorodrigo90/ratatui-themekit.git
cd ratatui-themekit
cargo build
cargo test --all-features
```

## Quality Gates (before every commit)

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps --all-features
cargo deny check licenses bans sources
```

Or use the validate script: `./scripts/validate.sh`

## Adding a New Theme

1. Create `src/themes/<name>.rs` with `pub const NAME: ThemeData = ThemeData { ... };`
2. In `src/themes/mod.rs`: add `mod`, `pub use`, and add to `BUILTIN_THEMES`
3. In `src/lib.rs` aliases module: add PascalCase alias
4. Tests auto-validate all themes in the registry
5. Update README theme table
6. Regenerate screenshots: `./scripts/generate-screenshots.sh`

## Adding a New Builder

1. Create `src/builders/<name>.rs` with the builder struct + tests
2. In `src/builders/mod.rs`: add `mod`, `pub use`, and add `ThemeExt` method
3. In `src/lib.rs`: add to the re-export list
4. Update README Builders section
5. Add to `examples/showcase.rs` so it appears in screenshots
6. Regenerate screenshots: `./scripts/generate-screenshots.sh`

## Semver Policy

This crate follows [Semantic Versioning](https://semver.org/):

| Change | Version bump | Examples |
|--------|-------------|----------|
| **Breaking** | Major (1.0 → 2.0) | Remove/rename public API, change trait methods, change function signatures |
| **Feature** | Minor (0.4 → 0.5) | New builders, new themes, new ThemeExt methods, new style bundles |
| **Fix** | Patch (0.4.0 → 0.4.1) | Bug fixes, doc corrections, dependency updates |

**What counts as breaking:**
- Removing or renaming a public struct, trait, method, or function
- Changing required methods on `Theme` trait (adding = breaking!)
- Changing `ThemeData` struct fields
- Removing a built-in theme from `BUILTIN_THEMES`
- Changing default theme behavior

**What is NOT breaking:**
- Adding new `ThemeExt` methods (trait has blanket impl)
- Adding new builders or style bundles
- Adding new themes to `BUILTIN_THEMES`
- Adding new derived methods to `Theme` trait (with defaults)
- Adding new optional fields to `CustomTheme` (with serde defaults)

## Release Checklist

Before creating a tag, verify EVERY item. Do not skip steps.

### 1. Version bump

- [ ] `Cargo.toml` → `version = "X.Y.Z"`
- [ ] Verify semver is correct (see policy above)

### 2. Quality gates

```bash
cargo fmt --check            # zero formatting issues
cargo clippy --all-targets --all-features  # zero warnings
cargo test --all-features    # all tests pass
cargo doc --no-deps --all-features  # no doc warnings
cargo deny check             # licenses + bans + sources
```

### 3. Showcase & screenshots

```bash
cargo run --example showcase --release   # verify visually — all builders render
./scripts/generate-screenshots.sh        # regenerate PNGs + GIF
```

- [ ] Showcase shows correct version in header and footer
- [ ] All themes render correctly in PNGs
- [ ] GIF shows smooth theme switching with animations

### 4. Documentation

- [ ] `README.md` — builders section matches current API
- [ ] `README.md` — theme table matches `BUILTIN_THEMES`
- [ ] `README.md` — GIF at top is current
- [ ] `examples/README.md` — gallery has all theme PNGs
- [ ] `CLAUDE.md` — architecture matches file structure
- [ ] `AGENTS.md` — architecture matches file structure
- [ ] `CONTRIBUTING.md` — release checklist is current
- [ ] `site/public/llms.txt` — links are valid (if applicable)

### 5. Commit & tag

```bash
git add -A
git commit -m "feat: vX.Y.Z — <summary>"
git push origin main
git tag vX.Y.Z
git push origin vX.Y.Z
```

### 6. Verify release

- [ ] GitHub Actions: Release workflow passes (validate → publish → GitHub Release)
- [ ] [crates.io/crates/ratatui-themekit](https://crates.io/crates/ratatui-themekit) shows new version
- [ ] [docs.rs/ratatui-themekit](https://docs.rs/ratatui-themekit) builds successfully
- [ ] GitHub Release has correct changelog

### 7. Downstream update

- [ ] Ralph Engine: update `ratatui-themekit` version in `Cargo.toml`
- [ ] Ralph Engine: update re-exports in `theme.rs` if new builders added
- [ ] Ralph Engine: run full test suite

## Code Standards

- Rust 2024 edition, clippy pedantic
- All public items documented with `///`
- `#[must_use]` on builder methods and pure functions
- Tests colocated in each file
- Files max 450 lines, single responsibility
- No git co-author footers
- Conventional commits: `type(scope): description`

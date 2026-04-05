# AGENTS.md

Operational contract for AI coding assistants working on this repository.

## Quality Gates

Every change must pass before commit:

1. `cargo fmt` — zero formatting issues
2. `cargo clippy --all-targets` — zero warnings
3. `cargo test` — all tests pass
4. `cargo doc` — no doc warnings

## Commit Style

Conventional commits: `type(scope): description`

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `ci`

## Code Standards

- Rust 2024 edition
- Clippy pedantic enabled
- All public items documented
- Tests for every public function
- NO_COLOR compliance mandatory

#!/usr/bin/env bash
set -euo pipefail

# ── ratatui-themekit validation script ──────────────────────────
# Usage:
#   ./scripts/validate.sh                 # all checks (local mode)
#   ./scripts/validate.sh --mode hook     # pre-push checks (faster)
#   ./scripts/validate.sh --mode ci       # CI checks (full)
#   ./scripts/validate.sh --checks fmt,test  # specific checks only

MODE="local"
CHECKS=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode) MODE="$2"; shift 2 ;;
    --checks) CHECKS="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 1 ;;
  esac
done

default_checks_for_mode() {
  case "$1" in
    hook)    echo "fmt,clippy,test,deny,rustdoc" ;;
    ci)      echo "fmt,clippy,test,deny,audit,rustdoc" ;;
    local)   echo "fmt,clippy,test,deny,rustdoc" ;;
    release) echo "fmt,clippy,test,deny,audit,rustdoc" ;;
    *) echo "unknown mode: $1" >&2; exit 1 ;;
  esac
}

if [[ -z "$CHECKS" ]]; then
  CHECKS="$(default_checks_for_mode "$MODE")"
fi

want() { [[ ",${CHECKS}," == *",$1,"* ]]; }

PASS=0
FAIL=0

run_check() {
  local name="$1"
  shift
  printf "\n── %s ──\n" "$name"
  if "$@"; then
    printf "✓ %s passed\n" "$name"
    PASS=$((PASS + 1))
  else
    printf "✗ %s FAILED\n" "$name"
    FAIL=$((FAIL + 1))
  fi
}

# ── Checks ──────────────────────────────────────────────────────

if want fmt; then
  run_check "fmt" cargo fmt --check
fi

if want clippy; then
  run_check "clippy" cargo clippy --all-targets -- -D warnings
fi

if want test; then
  run_check "test" cargo test --all-targets
fi

if want deny; then
  if command -v cargo-deny &>/dev/null; then
    run_check "deny (licenses)" cargo deny check licenses
    run_check "deny (bans)" cargo deny check bans
    run_check "deny (sources)" cargo deny check sources
  else
    printf "\n── deny ──\nskipped (cargo-deny not installed)\n"
  fi
fi

if want audit; then
  if command -v cargo-audit &>/dev/null; then
    run_check "audit" cargo audit
  else
    printf "\n── audit ──\nskipped (cargo-audit not installed)\n"
  fi
fi

if want rustdoc; then
  run_check "rustdoc" cargo doc --no-deps
fi

# ── Summary ─────────────────────────────────────────────────────

printf "\n══════════════════════════════════════\n"
printf "  ✓ %d passed  ✗ %d failed\n" "$PASS" "$FAIL"
printf "══════════════════════════════════════\n"

if [[ "$FAIL" -gt 0 ]]; then
  exit 1
fi

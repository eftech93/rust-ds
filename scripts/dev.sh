#!/usr/bin/env bash
# Single entry point for local development tasks.
#
#   scripts/dev.sh <command> [args]
#
# Run `scripts/dev.sh help` for the list of commands.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CRATE="dioxus-ui-system"
DX_VERSION="0.7"
IOS_BUNDLE_ID="com.example.ExampleMobile"
IOS_PROCESS="ExampleMobile"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"

if [[ -t 1 ]]; then
  BOLD=$'\033[1m'; RED=$'\033[31m'; GREEN=$'\033[32m'; YELLOW=$'\033[33m'; NC=$'\033[0m'
else
  BOLD=""; RED=""; GREEN=""; YELLOW=""; NC=""
fi

step() { echo "${BOLD}==> $*${NC}"; }
ok()   { echo "  ${GREEN}✓${NC} $*"; }
warn() { echo "  ${YELLOW}!${NC} $*"; }
fail() { echo "  ${RED}✗${NC} $*"; }
die()  { echo "${RED}error:${NC} $*" >&2; exit 1; }

need_dx() {
  command -v dx >/dev/null || die "dx not found. Install: cargo binstall dioxus-cli@$DX_VERSION (or cargo install dioxus-cli@$DX_VERSION)"
}

booted_sim() {
  xcrun simctl list devices 2>/dev/null | grep Booted |
    grep -oE '[A-F0-9]{8}-([A-F0-9]{4}-){3}[A-F0-9]{12}' | head -1
}

# ---------------------------------------------------------------- setup ----

cmd_doctor() {
  step "Toolchain"
  for c in rustc cargo rustup; do
    if command -v "$c" >/dev/null; then ok "$c $("$c" --version 2>/dev/null | head -1 | cut -d' ' -f2)"; else fail "$c missing"; fi
  done
  if command -v dx >/dev/null; then
    v=$(dx --version | awk '{print $2}')
    [[ "$v" == "$DX_VERSION"* ]] && ok "dx $v" || warn "dx $v (project uses $DX_VERSION.x)"
  else
    fail "dx missing: cargo binstall dioxus-cli@$DX_VERSION"
  fi

  installed=$(rustup target list --installed 2>/dev/null || true)
  check_target() { grep -qx "$1" <<<"$installed" && ok "target $1" || warn "target $1 missing: rustup target add $1"; }

  step "Web"
  check_target wasm32-unknown-unknown

  step "Android"
  [[ -n "${ANDROID_HOME:-}" ]] && ok "ANDROID_HOME=$ANDROID_HOME" || warn "ANDROID_HOME not set"
  ndk="${ANDROID_NDK_HOME:-${NDK_HOME:-}}"
  [[ -n "$ndk" ]] && ok "NDK $ndk" || warn "ANDROID_NDK_HOME not set"
  for t in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android; do check_target "$t"; done

  if [[ "$OSTYPE" == darwin* ]]; then
    step "iOS"
    xcode-select -p >/dev/null 2>&1 && ok "Xcode $(xcode-select -p)" || warn "Xcode command line tools missing: xcode-select --install"
    for t in aarch64-apple-ios aarch64-apple-ios-sim; do check_target "$t"; done
  fi
}

# ------------------------------------------------------------ quality ----

cmd_fmt()    { cargo fmt --all; }
cmd_lint()   { cargo clippy --workspace --all-targets -- -D warnings; }
cmd_test()   { cargo test -p "$CRATE" "$@"; }

# Same checks as .github/workflows/ci.yml (minus the cross-OS matrix).
cmd_ci() {
  step "fmt";     cargo fmt --all --check
  step "clippy";  cmd_lint
  step "test";    cmd_test
  step "wasm";    cargo check -p "$CRATE" --target wasm32-unknown-unknown
                  cargo check -p dioxus-ui-docs --target wasm32-unknown-unknown
  step "package"; cargo publish -p "$CRATE" --dry-run --allow-dirty
  ok "all checks passed"
}

# ---------------------------------------------------------------- run ----

cmd_docs() {
  need_dx
  case "${1:-serve}" in
    serve) shift || true; (cd docs-app && dx serve --platform web "$@") ;;
    build)
      (cd docs-app && dx build --platform web --release)
      out="$TARGET_DIR/dx/dioxus-ui-docs/release/web/public"
      cp "$out/index.html" "$out/404.html"
      ok "built: $out" ;;
    *) die "usage: dev.sh docs [serve|build]" ;;
  esac
}

cmd_web()     { need_dx; dx serve -p example-web-csr --platform web "$@"; }
cmd_ssr()     { cargo run -p example-web-ssr "$@"; }
cmd_desktop() { cargo run -p example-desktop "$@"; }
cmd_android() { need_dx; (cd examples/mobile && dx serve --platform android "$@"); }

cmd_ios() {
  [[ "$OSTYPE" == darwin* ]] || die "iOS requires macOS"
  local sub="${1:-run}"; shift || true
  case "$sub" in
    run) need_dx; (cd examples/mobile && dx serve --platform ios "$@") ;;
    sim)
      # Boot a simulator (first available iPhone unless one is booted) and open it.
      if [[ -z "$(booted_sim)" ]]; then
        dev=$(xcrun simctl list devices available | grep -m1 iPhone |
          grep -oE '[A-F0-9]{8}-([A-F0-9]{4}-){3}[A-F0-9]{12}') || die "no iPhone simulator; create one in Xcode"
        xcrun simctl boot "$dev"
      fi
      open -a Simulator; ok "booted $(booted_sim)" ;;
    logs)
      uuid=$(booted_sim); [[ -n "$uuid" ]] || die "no booted simulator"
      xcrun simctl spawn "$uuid" log stream --level debug \
        --predicate "process == \"$IOS_PROCESS\" OR eventMessage CONTAINS \"panic\"" ;;
    status)
      uuid=$(booted_sim); [[ -n "$uuid" ]] || die "no booted simulator"
      ok "simulator $uuid"
      if path=$(xcrun simctl get_app_container "$uuid" "$IOS_BUNDLE_ID" 2>/dev/null); then ok "installed: $path"; else warn "$IOS_BUNDLE_ID not installed"; fi
      ls -t "$HOME/Library/Logs/DiagnosticReports" 2>/dev/null | grep -i "$IOS_PROCESS" | head -3 | sed 's/^/  crash report: /' || true ;;
    reset)
      warn "erasing all simulators"
      xcrun simctl shutdown all 2>/dev/null || true
      xcrun simctl erase all
      killall Simulator 2>/dev/null || true
      ok "simulators reset" ;;
    *) die "usage: dev.sh ios [run|sim|logs|status|reset]" ;;
  esac
}

# ------------------------------------------------------------ release ----

cmd_version() {
  cargo metadata --no-deps --format-version 1 |
    python3 -c "import json,sys; print(next(p['version'] for p in json.load(sys.stdin)['packages'] if p['name']=='$CRATE'))"
}

# Bump the workspace + crate version and create the matching release branch.
cmd_bump() {
  local new="${1:-}"
  [[ "$new" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "usage: dev.sh bump X.Y.Z"
  local old; old=$(cmd_version)
  sed -i.bak -E "s/^version = \"$old\"/version = \"$new\"/" Cargo.toml "$CRATE/Cargo.toml"
  rm -f Cargo.toml.bak "$CRATE/Cargo.toml.bak"
  ok "$old -> $new"
  if [[ "$(git rev-parse --abbrev-ref HEAD)" != "release/v$new" ]]; then
    git switch -c "release/v$new"
    ok "created branch release/v$new"
  fi
  echo "Next: update CHANGELOG.md, commit, push, open a PR to main. Merging it publishes v$new."
}

# Manual fallback for the release workflow.
cmd_publish() {
  cargo whoami >/dev/null 2>&1 || die "not logged in to crates.io (cargo login)"
  cmd_ci
  local v; v=$(cmd_version)
  read -r -p "Publish $CRATE $v to crates.io? (yes/no) " answer
  [[ "$answer" == yes ]] || { echo "Cancelled."; exit 0; }
  cargo publish -p "$CRATE"
}

cmd_help() {
  cat <<EOF
Usage: scripts/dev.sh <command> [args]

Setup
  doctor               Check toolchain, dx, wasm/iOS/Android targets

Quality
  fmt                  Format the workspace
  lint                 Clippy (warnings are errors)
  test [args]          Run library tests
  ci                   Everything CI runs: fmt, clippy, test, wasm check, package

Run
  docs [serve|build]   Docs site (default: serve)
  web                  Web CSR example (dx serve)
  ssr                  Web SSR example
  desktop              Desktop example
  android              Mobile example on Android
  ios [run|sim|logs|status|reset]
                       Mobile example on iOS / simulator helpers (default: run)

Release
  version              Print the crate version
  bump X.Y.Z           Bump versions and create branch release/vX.Y.Z
  publish              Manual crates.io publish (fallback for CI)

Set CARGO_TARGET_DIR to build elsewhere (e.g. an external drive).
EOF
}

cmd="${1:-help}"; shift || true
case "$cmd" in
  doctor|fmt|lint|test|ci|docs|web|ssr|desktop|android|ios|version|bump|publish|help) "cmd_$cmd" "$@" ;;
  -h|--help) cmd_help ;;
  *) cmd_help; die "unknown command: $cmd" ;;
esac

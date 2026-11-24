# Changelog

All notable changes to `dioxus-ui-system` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/) (pre-1.0: minor bumps may break APIs).

## [Unreleased] - 0.0.8

0.0.7 was never published to crates.io; its changes are included here.

### Changed

- Migrated the library, docs site and examples to Dioxus 0.7 (#39).
- `use_theme()` no longer panics without a `ThemeProvider`; it falls back to a
  component-local light theme. `use_style` follows the same fallback (#16).
- `ToastProvider` now shares its `ToastManager` through context, so `use_toast()` and the
  provider see the same state (#34).

### Added

- `try_use_theme()` returns `Option<ThemeContext>` for detecting a provider (#16).
- `Tooltip` `hide_delay` prop (#15).
- Branch-based release workflow (`.github/workflows/release.yml`) on a self-hosted
  runner: pushing `release/vX.Y.Z` runs a dry run; merging it into `main` publishes to
  crates.io, records a GitHub Release and deploys the docs to GitHub Pages (#40).
- `scripts/dev.sh`: one script for setup checks, CI checks, running the docs/examples, iOS
  simulator helpers, version bumps and manual publishing. It replaces the 15
  `examples/*.sh` scripts, `scripts/publish.sh` and `docs-app/build.sh`.

### Fixed

- `Style` builder: `flex_grow`, `flex_shrink`, `flex_1`, `border_style`,
  `overflow_x_auto`, `overflow_y_auto` and `custom` now emit real CSS properties instead of
  being appended to the `transform` value (#12).
- `Separator` renders `role`/`aria-orientation` as attributes instead of writing them into
  the style string (#13).
- `Tooltip` honours its show `delay` (#15).
- Removed panicking `unwrap()`s in Tour, Sparkline, Avatar, InputGroup, Breadcrumb, Header
  and RichText; Tour no longer calls hooks after an early return (#35).
- CI: fmt, clippy (`-D warnings`), cross-platform tests, wasm check and package dry-run (#40).

## [0.0.6] - 2026-03-28

### Fixed

- Code review cleanups and compiler warning fixes.

## [0.0.5] - 2026-03-26

### Added

- Additional components and documentation updates.
- Opt-in theme persistence: `ThemeProvider { persist_theme: true, .. }` (off by default).
- Global component defaults via `ComponentConfig::builder()`.

## [0.0.1] - [0.0.4] - 2026-03-16 to 2026-03-18

Initial releases. See the git history for details.

[Unreleased]: https://github.com/eftech93/rust-ds/compare/de61bd6...HEAD
[0.0.6]: https://crates.io/crates/dioxus-ui-system/0.0.6
[0.0.5]: https://crates.io/crates/dioxus-ui-system/0.0.5

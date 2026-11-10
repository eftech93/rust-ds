# Contributing

Thanks for helping out. Bugs and planned work are tracked in
[GitHub issues](https://github.com/eftech93/rust-ds/issues); please open or comment on an
issue before starting larger changes.

## Development setup

- Rust: the toolchain is pinned in `rust-toolchain.toml` (rustup installs it automatically,
  with `clippy`, `rustfmt` and the wasm target), so local runs, CI and the release runner use
  the same clippy. The crate's MSRV for users is `rust-version` (`1.80`) in `Cargo.toml`.
- Dioxus CLI 0.7 for the docs site and web/mobile examples:
  `cargo binstall dioxus-cli@0.7` (or `cargo install dioxus-cli --version "^0.7" --locked`)
- Linux only, for the desktop/mobile examples (needed by `cargo clippy --workspace`):
  `sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev libayatana-appindicator3-dev`

## Project structure

```
rust-ds/
├── Cargo.toml              # workspace: members, shared version and metadata
├── dioxus-ui-system/       # the published crate
│   └── src/
│       ├── lib.rs          # module declarations and `prelude`
│       ├── config.rs       # global component defaults (ComponentConfig)
│       ├── theme/          # tokens.rs (ThemeTokens, Color, scales), context.rs (ThemeProvider, use_theme)
│       ├── styles/         # builder.rs (Style), cx!/style_if! macros
│       ├── atoms/          # Button, Input, Label, Icon, ...
│       ├── molecules/      # Card, Dialog, Tooltip, Toast, ...
│       └── organisms/      # Header, DataTable, Tabs, ..., charts/
├── docs-app/               # documentation site (crate `dioxus-ui-docs`)
├── examples/               # shared, web-csr, web-ssr, desktop, mobile (see examples/README.md)
├── scripts/dev.sh          # one entry point for local tasks (run `scripts/dev.sh help`)
└── .github/workflows/      # ci.yml, release.yml
```

Library code goes in `dioxus-ui-system/src/<layer>/`, is exported from that layer's
`mod.rs`, and, if it is public API, re-exported from `prelude` in `lib.rs`. Use theme tokens
(`use_theme()` / `use_style`) instead of hard-coded colors, and accept `style`/`class`
props where it makes sense.

## Running locally

Everything goes through `scripts/dev.sh` (`scripts/dev.sh help` lists the commands):

```bash
scripts/dev.sh doctor      # check toolchain, dx, wasm/iOS/Android targets
scripts/dev.sh docs        # docs site with hot reload (http://localhost:8080)
scripts/dev.sh web         # web CSR example
scripts/dev.sh ssr         # web SSR example
scripts/dev.sh desktop     # desktop example
scripts/dev.sh ios         # mobile example on the iOS simulator (also: ios sim|logs|status|reset)
scripts/dev.sh android     # mobile example on Android
```

Set `CARGO_TARGET_DIR` to build somewhere else, for example an external drive.

Mobile setup and more detail are in [examples/README.md](examples/README.md).

## Checks

CI (`.github/workflows/ci.yml`) runs these on every push to `main`/`release/**` and on
pull requests. `scripts/dev.sh ci` runs them all locally; run it before opening a PR:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p dioxus-ui-system
cargo check -p dioxus-ui-system --target wasm32-unknown-unknown
cargo check -p dioxus-ui-docs --target wasm32-unknown-unknown
cargo publish -p dioxus-ui-system --dry-run
```

Tests run on Linux, macOS and Windows; only the library crate is tested because the
example apps need platform webview SDKs.

## Pull requests

- Branch from `main`; release branches are named `release/vX.Y.Z`.
- Reference the issue (`Fixes #NN`) and add an entry under `Unreleased` in
  [CHANGELOG.md](CHANGELOG.md) for user-visible changes.
- Keep `cargo fmt` and `clippy -D warnings` clean.

## Releasing

Releases are branch-based and automated by `.github/workflows/release.yml`, which runs on
a self-hosted runner (label `self-hosted`, or whatever the repository variable
`RELEASE_RUNNER` is set to). There is no manual tagging: merging the release branch is
the release.

### Checklist

1. Create the release branch and bump the versions (root `Cargo.toml` and
   `dioxus-ui-system/Cargo.toml`):
   ```bash
   scripts/dev.sh bump 0.0.9      # creates release/v0.0.9
   ```
2. Update the install snippets in `README.md` and `dioxus-ui-system/README.md`, and move
   the `Unreleased` notes in `CHANGELOG.md` under the new version and date.
3. Push the branch. Every push to `release/vX.Y.Z` runs the release checks as a dry run;
   the branch name must equal `v<crate version>` or the run fails.
4. Open a PR to `main`. Merging it publishes `X.Y.Z`.
5. Check https://crates.io/crates/dioxus-ui-system, https://docs.rs/dioxus-ui-system and
   https://eftech93.github.io/rust-ds/.

The workflow can also be started from Actions → Release → Run workflow. That is a dry run
(verify and package only) unless you untick *Dry run*.

### What the workflow does

| Trigger | Result |
|---|---|
| push to `release/vX.Y.Z` | verify only (dry run) |
| PR from `release/vX.Y.Z` merged into `main` | verify, publish, GitHub Release, docs deploy |
| PR closed without merging, or from another branch | nothing |

1. **verify**: installs Rust (and `wasm32-unknown-unknown`) if missing, checks the branch
   matches the crate version, then runs fmt, clippy, tests, the wasm check and
   `cargo publish --dry-run` for `dioxus-ui-system`.
2. **publish-crate**: `cargo publish`; skipped if that version is already on crates.io,
   so re-running a release is safe.
3. **github-release**: records release `vX.Y.Z` (and its tag) on the merge commit, with
   generated notes and crates.io/docs.rs links. It's skipped if it already exists.
4. **build-docs / deploy-docs**: builds `docs-app` with `dx` 0.7 using base path
   `/rust-ds/`, copies `index.html` to `404.html` for SPA routing, and deploys it to
   GitHub Pages via Actions.

### One-time setup

| What | Where | Value |
|---|---|---|
| `CARGO_REGISTRY_TOKEN` | Settings → Secrets and variables → Actions → Secrets | crates.io API token with the `publish-update` scope, restricted to the `dioxus-ui-system` crate |
| `RELEASE_RUNNER` (optional) | Settings → Secrets and variables → Actions → Variables | Runner label if not `self-hosted` |
| Pages source | Settings → Pages → Build and deployment | GitHub Actions |

`GITHUB_TOKEN` is provided automatically. The runner needs `git`, `curl`, `python3` and a
C toolchain; Rust (via rustup) and `dioxus-cli` 0.7 are installed on first run if missing.
The crate publish uses the `release` environment and Pages uses `github-pages`, so you can
add protection rules there.

### Manual fallback

```bash
cargo login                                   # token from https://crates.io/settings/tokens
cargo publish -p dioxus-ui-system --dry-run
cargo publish -p dioxus-ui-system             # or scripts/dev.sh publish
```

Common errors: "already exists" means the version was published, so bump it;
"unauthorized" means the token is missing or expired; "dirty" means commit first.

## Docs site

`docs-app/` is a Dioxus web app (crate `dioxus-ui-docs`) with a page per component plus
theme and guide pages. Shared page building blocks (`DocPage`, `ExampleBox`, `CodeBlock`,
`PropsTable`) live in `docs-app/src/docs_ui/`.

```bash
scripts/dev.sh docs          # dev server with hot reload
scripts/dev.sh docs build    # output: target/dx/dioxus-ui-docs/release/web/public/
```

`base_path` in `docs-app/Dioxus.toml` stays commented out for local development; the
release workflow sets it to `/rust-ds/` at build time. GitHub Pages has no SPA routing, so
the deployed site includes a copy of `index.html` as `404.html`; deep links such as
`/rust-ds/atoms/button` rely on it. If assets 404 after a deploy, check the base path
(case-sensitive, trailing slash).

Deploys happen as part of a release. To redeploy the docs without publishing, run the
Release workflow manually with *Dry run* unticked (the crate publish step is skipped when
the version already exists on crates.io).

## Misc

Build outputs are ignored by the root `.gitignore`; examples with platform-specific
artifacts (`examples/mobile`, `examples/desktop`, `examples/web-csr`) have their own
`.gitignore`.

# dioxus-ui-system Roadmap

**Last updated**: 2026-08-14
**Current version**: 0.0.7 (unreleased) — last GitHub release: v0.0.6

## Release Process

- **One issue = one version = one GitHub Release.**
- Each version has a GitHub milestone; when the issue is done, the milestone closes and a
  GitHub Release is created (tag `vX.Y.Z`, release notes generated from the issue).
- Release branches follow the existing convention: `release/vX.Y.Z`.
- A release is only cut when CI is green: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`.

## Current State (2026-08-14)

| Check | Status |
|---|---|
| Workspace compiles (lib + examples + docs-app) | pass |
| Unit tests (37) | pass |
| Clippy | fail — 257 warnings |
| `cargo fmt --check` | fail |
| Doc tests | all 34 `ignore`d |

Known defects: Style builder emits invalid CSS via the `transform` field hack,
Separator writes aria/role into the style string, Dialog/Tabs/Dropdown lack a11y and
keyboard support, `use_theme()` panics without a provider, tooltip delay unimplemented,
only 12/82 component files have (trivial) tests, widespread hardcoded hex colors bypassing theme tokens in atoms/molecules/organisms, disconnected Toast signal state in ToastProvider, unsafe unwraps in Tour/Sparkline, and docs-app crashes without ErrorBoundary trapping.

## Phase 1 — Stabilization (patch releases)

| Version | Issue | Type |
|---|---|---|
| v0.0.8 | Fix all clippy warnings; enforce `-D warnings` in CI | bug |
| v0.0.9 | Apply `cargo fmt`; enforce format check in CI | chore |
| v0.0.10 | Fix Style builder methods emitting invalid CSS (transform hack) | bug |
| v0.0.11 | Fix Separator aria/role attributes written into style string | bug |
| v0.0.12 | Replace fn-pointer `PartialEq` in `DateRangePreset` | bug |
| v0.0.13 | Implement Tooltip show/hide delay | bug |
| v0.0.14 | Non-panicking `use_theme` fallback | enhancement |
| v0.0.15 | Eliminate unsafe `.unwrap()` panics in Tour, Sparkline, and Header | bug |
| v0.0.16 | Fix ToastProvider and use_toast state synchronization via context provider | bug |
| v0.0.17 | Theme integration: replace hardcoded colors with design tokens across atom components | bug |
| v0.0.18 | Theme integration: replace hardcoded colors and surface styles in molecules and organisms | bug |
| v0.0.19 | docs-app: add ErrorBoundary and NotFound route to prevent navigation lockup on errors | bug |
| v0.0.20 | docs-app: fix mobile navigation menu completeness, Tour demo rendering, and Theme Builder auto-apply | bug |

## Phase 2 — API Normalization (breaking, pre-1.0)

| Version | Issue | Type |
|---|---|---|
| v0.1.0 | BREAKING: normalize form props (`error`, `label`, `placeholder`, `title`) | breaking-change |
| v0.2.0 | BREAKING: normalize event handler API (`on_click` naming, optionality) | breaking-change |

## Phase 3 — Accessibility & Keyboard

| Version | Issue | Type |
|---|---|---|
| v0.3.0 | Dialog family a11y: `role="dialog"`, `aria-modal`, Escape, focus trap | a11y |
| v0.4.0 | Tabs a11y: `tablist`/`tab` roles, `aria-selected`, arrow-key navigation | a11y |
| v0.5.0 | DropdownMenu/ContextMenu a11y + keyboard navigation | a11y |
| v0.6.0 | Remaining aria: progressbar, radiogroup, multiselect listbox, ThemeToggle label | a11y |

## Phase 4 — Quality & Docs

| Version | Issue | Type |
|---|---|---|
| v0.7.0 | Test infrastructure + rendering tests for core components | testing |
| v0.8.0 | Enable doc tests (remove `ignore`) | testing |
| v0.9.0 | docs-app cleanup: consolidate `*_new` pages, remove dead dirs | docs-app |
| v0.10.0 | README visual proof: real screenshots | documentation |

## Phase 5 — Stable

| Version | Issue | Type |
|---|---|---|
| v1.0.0 | Stable release: API freeze, CHANGELOG, crates.io + docs.rs publish | release |

## Backlog (unscheduled)

- CSS custom properties for runtime theming and component-level overrides
- Builder-pattern component API exploration (`Button::builder()...`)
- Move docs components into the library (`DocPage`, `ExampleBox`, `CodeBlock`)
- Keyboard navigation for Accordion, Tree, Carousel, Popover
- Rich text `ToolbarButton` props cleanup (only non-Optional `style` prop in the codebase)

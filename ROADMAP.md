# Roadmap

[GitHub issues](https://github.com/eftech93/rust-ds/issues) are the source of truth for
planned work; this page is only a summary. Findings from earlier code reviews are tracked
there as issues too. Shipped changes are listed in [CHANGELOG.md](CHANGELOG.md).

## Current: 0.0.8

Dioxus 0.7 migration, stabilization fixes (Style builder CSS, Separator a11y, Tooltip
delays, non-panicking `use_theme`, Toast context, unwrap removal) and the automated
release pipeline. See the changelog for details.

## Next: stabilization (0.0.x)

- Clippy clean with `-D warnings` across the workspace (#10)
- Replace fn-pointer `PartialEq` in `DateRangePreset` (#14)
- Replace hard-coded colors with theme tokens in atoms (#32) and molecules/organisms (#33)
- docs-app: ErrorBoundary and NotFound route (#36); mobile nav, Tour demo, Theme Builder
  fixes (#37); consolidate the `*_new` pages (#25)

## Then: API normalization (0.x, breaking)

- Consistent form props: `error`, `label`, `placeholder`, `title` (#17)
- Consistent event handler naming and optionality, e.g. `on_change` vs `onchange` (#18)

## Accessibility and keyboard support

- Dialog family: `role="dialog"`, `aria-modal`, Escape, focus trap (#19)
- Tabs roles and arrow-key navigation (#20)
- DropdownMenu / ContextMenu (#21)
- Remaining roles: progressbar, radiogroup, multiselect, ThemeToggle label (#22)
- Keyboard navigation for Accordion, Tree, Carousel, Popover (#31)

## Quality and docs

- Rendering tests for core components (#23)
- Run doc tests instead of marking them `ignore` (#24)
- Screenshots in the README (#26)

## 1.0

API freeze, stable release on crates.io and docs.rs (#27).

## Ideas (unscheduled)

- CSS custom properties for runtime theming and component-level overrides (#28)
- Builder-style component API, e.g. `Button::builder()` (#29)
- Move docs building blocks (`DocPage`, `ExampleBox`, `CodeBlock`) into the library (#30)
- Evaluate tailwind-rs (#38)
- Clean up `ToolbarButton` props in rich text (its `style` prop is the only non-optional one)

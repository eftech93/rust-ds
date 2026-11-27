# Changelog

All notable changes to `dioxus-ui-system` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/) (pre-1.0: minor bumps may break APIs).

## [0.0.10] - Unreleased

Accessibility release (#19, #20, #21, #22, #31). **Not breaking:** every change only adds
roles, ARIA attributes, keyboard behaviour or optional props.

### Added

- `dioxus_ui_system::a11y`: shared helpers: `use_unique_id`, `focus_element`,
  `install_focus_trap` / `release_focus_trap`, and `next_index` / `next_enabled_index`
  for arrow-key / Home / End navigation.
- **Dialog, AlertDialog, Sheet, ConfirmationDialog family** (#19): `role="dialog"` /
  `"alertdialog"`, `aria-modal`, `aria-labelledby` / `aria-describedby` wired to the title
  and description. Escape closes, focus is trapped while open and restored on close, and
  close buttons have a "Close" label. Sheet's web-only `web_sys` Escape listener, which
  leaked a closure on every open, is replaced by a handler that also works on desktop and
  mobile. New optional props: `Dialog.alert`, `Dialog.aria_label`,
  `Dialog.aria_describedby`, `Sheet.aria_label`.
- **Popover** (#31): `aria-haspopup` / `aria-expanded` / `aria-controls` on the trigger,
  `role="dialog"` content, Escape closes, focus moves in and returns to the trigger.
  HoverCard's hidden card is `aria-hidden`.
- **Tabs / VerticalTabs** (#20): `tablist` / `tab` / `tabpanel` roles, `aria-selected`,
  `aria-controls`, `aria-labelledby`, roving tabindex. Arrow keys (Left/Right, or Up/Down
  for vertical), Home and End move to and activate a tab and skip disabled ones.
- **DropdownMenu** (#21): `aria-haspopup="menu"`, `aria-expanded`, `role="menu"` /
  `menuitem` / `separator`, `aria-disabled`. Enter, Space or ArrowDown opens the menu;
  arrows, Home and End move between items; Enter or Space selects; Escape closes and
  returns focus to the trigger.
- **Progress** (#22): `role="progressbar"` with `aria-valuenow` / `min` / `max` /
  `valuetext`, plus `aria-busy` when indeterminate. New optional props: `aria_label` and
  `value_text`.
- **RadioGroup** (#22): `role="radiogroup"` and `aria-orientation`. New optional props:
  `label` (a visible group label), `aria_label` and `id`.
- **MultiSelect** (atoms and molecules) (#22): combobox and listbox pattern with
  `aria-expanded`, `aria-activedescendant` and `aria-multiselectable`, plus keyboard
  support (open, move the highlight, toggle, Escape). New optional prop: `aria_label`.
- **ThemeToggle / ThemeSelector** (#22): an action label ("Switch to dark theme") and
  listbox roles.
- **Accordion** (#31): `aria-controls`, `role="region"` panels; Up/Down/Home/End move
  between headers.
- **Tree** (#31): the WAI-ARIA tree pattern: `aria-level` / `setsize` / `posinset`,
  roving tabindex, Up/Down, Right/Left expand-collapse-or-move, Home/End, and Enter/Space
  to select. New optional prop: `aria_label`.
- **Carousel** (#31): `aria-roledescription` region and slides ("n of m"), `aria-live`.
  Left/Right (Up/Down when vertical) and Home/End change slides, and `is_paused` is set
  while the carousel is focused. New optional props: `aria_label` (Carousel,
  SimpleCarousel, TouchCarousel) and `CarouselItem.total`.
- Docs site: an "Accessibility" guide and keyboard sections on the affected component
  pages.

### Fixed

- `Tabs` now renders its `children` (e.g. `TabPanel`); before, they were dropped.
- Tab buttons restyle when the active tab changes; a memo had kept the old state.
- Dialog and Sheet no longer call hooks conditionally when `open` changes.

## [0.0.9] - 2026-11-25

**Breaking:** one pass that makes component props consistent (#17, #18), so you
migrate once. The workspace now builds with zero warnings (`-D warnings` for build,
clippy and rustdoc).

### Migration guide

**General rules**

| Before | After |
|---|---|
| `onclick`, `onmouseenter`, `onmouseleave`, `onfocus`, `onblur`, `oninput` | `on_click`, `on_mouse_enter`, `on_mouse_leave`, `on_focus`, `on_blur`, `on_input` |
| Required `EventHandler<T>` props | `Option<EventHandler<T>>`, default `None`. Passing a closure still works. |
| Click/hover/focus payloads `MouseEvent` / `FocusEvent` | `()`. Write `move \|_\| ...` |
| `Input.oninput: FormEvent` | `on_input: String` (the current value) |
| `error: bool` (`Select`, `TextArea`, `OtpInput`) | `error: Option<String>`: the message, rendered below the field |
| Required `label` / `title` / `description` / `placeholder` / `style` `String` | `Option<String>`; `label: "x"` call sites keep compiling |

Native element attributes inside `rsx!` (`button { onclick: ... }`) are DOM listeners and
are unchanged. Only our components' props were renamed.

**Renamed handlers**

- `Button`, `IconButton`, `Box`, `Badge`, `Card`, `Sparkline`, `MobileMenuToggle`:
  `onclick: EventHandler<MouseEvent>` → `on_click: EventHandler<()>`. `UserMenuItem.onclick`
  (data struct field) → `on_click`.
- `Box`: `onmouseenter` / `onmouseleave` → `on_mouse_enter` / `on_mouse_leave` (payload `()`).
- `Input`: `onfocus` / `onblur` → `on_focus` / `on_blur` (payload `()`); `oninput` →
  `on_input: EventHandler<String>`.
- `Radio.on_change` now receives the radio's `value` (`String`) instead of `()`.

**Handlers that were required and are now optional**

`NumberInput`, `PasswordInput`, `RatingInput`, `Slider`, `RangeSlider`, `RadioGroup`,
`TagGroup`, `InputTag`, `Combobox`, `MultiCombobox`, `MultiSelect`, `OtpInput`,
`Pagination`, `PageSizeSelector`, `TimePicker`, `TimeInput`, `Accordion`, `Calendar`
(also `on_month_change`), `Tabs`, `VerticalTabs`, `ImageUploader` (`on_change`),
`CommandInput.on_value_change`, `CommandItem.on_select`, `DropdownMenu.on_select`,
`ActionListItem.on_click`, `Sheet.on_open_change`, `Toast.on_close`, `Dialog.on_close`,
`AlertDialog.on_close`/`on_confirm`, `ConfirmationDialog`, `DeleteConfirmDialog`,
`UnsavedChangesDialog`, `SignOutDialog` (all handlers), `ActionCard.on_action`,
`DualActionCard`/`ImageActionCard` `on_primary`/`on_secondary`, `PricingCard.on_cta`,
`FileUpload.on_upload`, `MenubarItem.on_click`, `MenubarCheckboxItem.on_checked_change`,
`Wizard.on_step_change`/`on_finish`, `Tour.on_open_change`, and the data_table
`Pagination.on_page_change`. `sonner::ToastAction.on_click` (data struct field) is now
`Option<EventHandler<()>>`. Construct it with `Some(EventHandler::new(...))`.

**`error` props**

- `Select.error`, `OtpInput.error`: `bool` → `Option<String>`.
- `TextArea.error`: `bool` → `Option<String>`, and `error_message` is removed:
  `error: true, error_message: Some(msg)` → `error: msg`.

**Content props that became optional (`Option<String>`)**

- `Spinner.label` (`None` still shows "Loading"), `StepLabel.label`, `InputGroup.label`,
  `StatCard.label`, `KanbanTag.label`, `MenubarTrigger.label`, `ActionListItem.label`
- `ListItem.title`, `AlertDialog.title`/`description`, `HoverCardHeader.title`,
  `PopoverHeader.title`, `AccordionItem2.title`, `ConfirmationDialog.title`, `Hero.title`
- `ActionCard`, `DualActionCard`, `ImageCard`, `ImageActionCard`, `HorizontalCard`:
  `title`/`description`; `NotificationCard`, `ExpandableCard`, `MediaCard`: `title`
- `DateRangePicker.placeholder`, `ImageUploader.placeholder` (`None` keeps the old
  default text)
- `rich_text::ToolbarButton`: `title` (`&'static str`) and `style` → `Option<String>`

### Docs and release

- Docs site: props tables and code samples match the 0.0.9 API, and a new
  "Migrating to 0.0.9" guide is at `/guides/migration`.
- Release workflow: publishing now runs on the push to `main` that the release PR's
  merge creates (only when the version is not on crates.io yet). This lets the
  GitHub Pages deploy pass the `github-pages` environment's main-only rule.

## [0.0.8] - 2026-11-24

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

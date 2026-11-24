# Changelog

All notable changes to `dioxus-ui-system` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/) (pre-1.0: minor bumps may break APIs).

## [0.0.9] - Unreleased

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

# dioxus-ui-system

[![Crates.io](https://img.shields.io/crates/v/dioxus-ui-system)](https://crates.io/crates/dioxus-ui-system)
[![docs.rs](https://docs.rs/dioxus-ui-system/badge.svg)](https://docs.rs/dioxus-ui-system)
[![License](https://img.shields.io/crates/l/dioxus-ui-system)](#license)

A pure Rust design system for [Dioxus](https://dioxuslabs.com/) 0.7. Components are
organized as atoms, molecules and organisms, styled from typed theme tokens with inline
styles (no CSS files), and run on web (WASM), desktop and mobile.

Live component gallery: https://eftech93.github.io/rust-ds/

## Installation

```toml
[dependencies]
dioxus = "0.7"
dioxus-ui-system = "0.0.8"
```

### Features

| Feature   | Default | Description |
|-----------|---------|-------------|
| `web`     | yes     | Enables `dioxus/web` plus `web-sys`/`wasm-bindgen` integrations (theme persistence via `localStorage`, clipboard, file inputs) |
| `desktop` | no      | Enables `dioxus/desktop` |
| `mobile`  | no      | Enables `dioxus/mobile` |
| `router`  | no      | Adds `dioxus-router` |

For non-web targets disable the defaults:

```toml
dioxus-ui-system = { version = "0.0.8", default-features = false, features = ["desktop"] }
```

## Quick start

```rust
use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut email = use_signal(String::new);

    rsx! {
        ThemeProvider {
            Card {
                variant: CardVariant::Elevated,
                CardHeader { title: "Sign up", subtitle: "It takes a minute" }
                CardContent {
                    InputGroup {
                        label: "Email",
                        value: email(),
                        placeholder: "you@example.com",
                        hint: "We'll never share your email.",
                        on_change: move |v| email.set(v),
                    }
                }
                CardFooter {
                    justify: CardFooterJustify::Between,
                    Button { variant: ButtonVariant::Ghost, "Cancel" }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| println!("submitted {}", email()),
                        "Continue"
                    }
                }
            }
        }
    }
}
```

Everything is re-exported from `dioxus_ui_system::prelude`; the modules `atoms`,
`molecules`, `organisms`, `theme`, `styles` and `config` are also public.

## Theming

Wrap the app in `ThemeProvider`:

```rust
rsx! {
    ThemeProvider {
        initial_theme: ThemeTokens::dark(), // optional, defaults to light
        persist_theme: true,                // optional, saves the choice to localStorage (web)
        App {}
    }
}
```

Presets: `ThemeTokens::light()`, `dark()`, `rose()`, `blue()`, `green()`, `violet()`,
`orange()`, or by name with `ThemeTokens::by_name("rose")`. Build a brand theme from one
color, or edit tokens directly:

```rust
use dioxus_ui_system::theme::{Color, ThemeTokens};

let brand = ThemeTokens::brand(Color::new(147, 51, 234), "purple");

let mut custom = ThemeTokens::light();
custom.colors.primary = Color::new(220, 38, 38);
```

`ThemeToggle` (light/dark) and `ThemeSelector` (preset picker) are ready-made controls.

Reading the theme inside a component:

```rust
#[component]
fn Panel() -> Element {
    let theme = use_theme();
    let bg = theme.tokens.read().colors.background.to_rgba();
    // theme.toggle_mode.call(()); theme.set_theme_by_name.call("blue".into());

    rsx! { div { style: "background-color: {bg}", "Content" } }
}
```

`use_theme()` does not panic when no `ThemeProvider` is mounted: it falls back to a
component-local light theme (changes made through it only affect that component). Use
`try_use_theme()` to get an `Option<ThemeContext>` when you need to detect the provider.
`use_style(|tokens| ...)` returns a memoized value derived from the current tokens.

## Style builder

`Style` is a fluent, Tailwind-like builder that produces an inline style string from
theme scales:

```rust
#[component]
fn Toolbar(children: Element) -> Element {
    let theme = use_theme();
    let t = theme.tokens.read();

    let style = Style::new()
        .flex()
        .items_center()
        .justify_between()
        .gap(&t.spacing, "md")
        .p(&t.spacing, "lg")
        .bg(&t.colors.muted)
        .rounded(&t.radius, "lg")
        .overflow_x_auto()
        .custom("backdrop-filter: blur(4px);")
        .build();

    rsx! { div { style: "{style}", {children} } }
}
```

Spacing sizes are `xs`, `sm`, `md`, `lg`, `xl`, `xxl`; radius sizes are `none`, `sm`,
`md`, `lg`, `xl`, `full`. The `cx!` and `style_if!` macros help with conditional
classes/styles.

## Customizing components

Most components accept `style: Option<String>` and `class: Option<String>` in addition to
their variants and sizes. App-wide defaults can be set once at startup:

```rust
ComponentConfig::builder()
    .button_size("lg")
    .card_padding("24px")
    .transitions(true)
    .build();
```

## Component catalog

**Atoms**: Button, IconButton, Input, PasswordInput, NumberInput, TextArea,
AutoResizeTextArea, InputTag, Select, Checkbox, Radio/RadioGroup, Switch, Toggle, Slider,
RangeSlider, Rating/RatingInput, DatePicker, Label, Heading, Paragraph, Blockquote,
Caption, Icon, Tag/TagGroup, Divider, Progress, Spinner, LoadingOverlay, Skeleton
variants, Step indicators, Box/HStack/VStack/Center/Spacer, AspectRatio

**Molecules**: Card, Badge, Alert, Avatar/AvatarGroup, Breadcrumb, Dialog, AlertDialog,
Sheet, DropdownMenu, ContextMenu, Popover, HoverCard, Tooltip, Toast/ToastProvider
(`use_toast`), Sonner, Combobox, Command palette, MultiSelect, OtpInput,
TimePicker/TimeInput, Pagination, Stepper, Collapsible, ScrollArea, Separator,
ToggleGroup, QrCode, InputGroup, ListItem, MediaObject

**Organisms**: Header, Footer, Layout, Hero, DataTable, Tabs/VerticalTabs, Accordion,
Cards (Action, Image, Media, Pricing, Profile, Stat, ...), Calendar, DateRangePicker,
Carousel, Tree, Timeline, Menubar, Kanban, Tour, Stepper/Wizard, FileUpload,
ImageUploader, RichTextEditor, NotificationCenter, ConfirmationDialog, Resizable panels

**Charts**: BarChart, LineChart, PieChart, DonutChart, GaugeChart, Sparkline

Built-in icon names for `Icon { name: ... }` include `check`, `x`, `plus`, `minus`,
`arrow-*`, `chevron-*`, `menu`, `search`, `user`, `settings`, `home`, `bell`, `heart`,
`star`, `trash`, `edit`, `copy`, `external-link`, `loading`, `info`, `warning`, `alert`,
`error`, `moon`, `sun`, `book`, `layout`, `sidebar`, `maximize`, `box`, `smartphone`,
`palette`.

Props and live examples for every component are in the
[documentation site](https://eftech93.github.io/rust-ds/) and on
[docs.rs](https://docs.rs/dioxus-ui-system).

## License

Licensed under either of MIT or Apache-2.0 at your option.
Source and issue tracker: https://github.com/eftech93/rust-ds

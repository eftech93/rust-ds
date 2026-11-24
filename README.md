# Dioxus UI System

[![CI](https://github.com/eftech93/rust-ds/actions/workflows/ci.yml/badge.svg)](https://github.com/eftech93/rust-ds/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/dioxus-ui-system)](https://crates.io/crates/dioxus-ui-system)
[![docs.rs](https://docs.rs/dioxus-ui-system/badge.svg)](https://docs.rs/dioxus-ui-system)
[![License](https://img.shields.io/crates/l/dioxus-ui-system)](#license)

A pure Rust design system for [Dioxus](https://dioxuslabs.com/) 0.7, organized with
Atomic Design (atoms, molecules, organisms). Styling is generated in Rust from typed
theme tokens, so there are no CSS files to ship. Works on web (WASM), desktop and mobile.

- Live documentation and component gallery: https://eftech93.github.io/rust-ds/
- Crate: https://crates.io/crates/dioxus-ui-system
- API reference: https://docs.rs/dioxus-ui-system

## Install

```toml
[dependencies]
dioxus = "0.7"
dioxus-ui-system = "0.0.8"
```

Cargo features (default: `web`):

| Feature   | Purpose                                     |
|-----------|---------------------------------------------|
| `web`     | WASM target, enables `web-sys` integrations (e.g. theme persistence) |
| `desktop` | Dioxus desktop renderer                     |
| `mobile`  | Dioxus mobile renderer                      |
| `router`  | Pulls in `dioxus-router`                    |

```toml
# Desktop app, no web dependencies
dioxus-ui-system = { version = "0.0.8", default-features = false, features = ["desktop"] }
```

## Example

```rust
use dioxus::prelude::*;
use dioxus_ui_system::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            Card {
                CardHeader { title: "Welcome", subtitle: "Built with pure Rust" }
                CardContent {
                    Button { variant: ButtonVariant::Primary, "Get started" }
                    ThemeToggle {}
                }
            }
        }
    }
}
```

See [dioxus-ui-system/README.md](dioxus-ui-system/README.md) for theming, the `Style`
builder and the full component catalog.

## Components

- **Atoms**: Button, IconButton, Input, PasswordInput, NumberInput, TextArea, Select,
  Checkbox, Radio, Switch, Toggle, Slider, Rating, DatePicker, Label, Heading, Paragraph,
  Icon, Tag, Divider, Progress, Spinner, Skeleton, Step, Box/HStack/VStack, AspectRatio
- **Molecules**: Card, Badge, Alert, Avatar, Breadcrumb, Dialog, AlertDialog, Sheet,
  DropdownMenu, ContextMenu, Popover, HoverCard, Tooltip, Toast/Sonner, Combobox, Command,
  MultiSelect, OtpInput, TimePicker, Pagination, Stepper, Collapsible, ScrollArea,
  Separator, ToggleGroup, QrCode, InputGroup, ListItem, MediaObject
- **Organisms**: Header, Footer, Layout, Hero, DataTable, Tabs, Accordion, Cards, Calendar,
  DateRangePicker, Carousel, Tree, Timeline, Menubar, Kanban, Tour, Wizard/Stepper,
  FileUpload, ImageUploader, RichText, NotificationCenter, ConfirmationDialog, Resizable
- **Charts**: BarChart, LineChart, PieChart/DonutChart, GaugeChart, Sparkline

Everything is re-exported from `dioxus_ui_system::prelude`.

## Repository layout

| Path                | What it is                                                     |
|---------------------|----------------------------------------------------------------|
| `dioxus-ui-system/` | The published library crate                                    |
| `docs-app/`         | Documentation site deployed to GitHub Pages                    |
| `examples/`         | Web (CSR/SSR), desktop and mobile example apps plus a shared showcase crate ([examples/README.md](examples/README.md)) |
| `scripts/`          | Manual publish helper                                          |
| `.github/workflows/`| CI (`ci.yml`) and release pipeline (`release.yml`)             |

## Project docs

- [CONTRIBUTING.md](CONTRIBUTING.md): development setup, testing, releasing, docs site
- [ROADMAP.md](ROADMAP.md): planned work (tracked in [GitHub issues](https://github.com/eftech93/rust-ds/issues))
- [CHANGELOG.md](CHANGELOG.md): release notes

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

Inspired by [shadcn/ui](https://ui.shadcn.com/) and [Radix UI](https://www.radix-ui.com/).

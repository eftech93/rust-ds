# Dioxus UI System

A pure Rust design system for Dioxus with Atomic Design principles. Build beautiful, type-safe user interfaces without touching CSS or JavaScript.

[![Crates.io](https://img.shields.io/crates/v/dioxus-ui-system)](https://crates.io/crates/dioxus-ui-system)
[![Documentation](https://docs.rs/dioxus-ui-system/badge.svg)](https://docs.rs/dioxus-ui-system)
[![License](https://img.shields.io/crates/l/dioxus-ui-system)](LICENSE)

## Features

- 🎨 **Atomic Design Architecture**: Components organized as Atoms, Molecules, and Organisms
- 🌗 **Type-Safe Theming**: Comprehensive theme system with light/dark/brand modes
- 🦀 **Pure Rust Styling**: No CSS files - all styles generated in Rust
- 🎯 **Tailwind-like API**: Fluent style builder for rapid UI development
- 📱 **Multi-Platform**: Works on Web (WASM), Desktop, and Mobile
- ⚡ **Zero Runtime CSS**: Inline styles for maximum performance
- 🔒 **Type Safety**: Catch styling errors at compile time

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-ui-system = "0.1"
dioxus = { version = "0.6", features = ["web"] }
```

Create your first UI:

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
                CardHeader {
                    title: "Welcome",
                    subtitle: Some("Get started with Dioxus UI".to_string()),
                }
                CardContent {
                    p { "Build beautiful UIs with pure Rust!" }
                    Button {
                        variant: ButtonVariant::Primary,
                        "Get Started"
                    }
                }
            }
        }
    }
}
```

## Architecture

### Atomic Design

Components are organized following Atomic Design principles:

```
Atoms          →  Buttons, Inputs, Labels, Icons
Molecules      →  InputGroup, Card, Badge
Organisms      →  Header, DataTable
```

### Theme System

The theme system provides type-safe design tokens:

```rust
use dioxus_ui_system::theme::{ThemeTokens, Color};

// Use preset themes
let light = ThemeTokens::light();
let dark = ThemeTokens::dark();

// Create custom brand theme
let brand = ThemeTokens::brand(Color::new(220, 38, 38), "acme");
```

### Style Builder

Build styles with a fluent, Tailwind-like API:

```rust
use dioxus_ui_system::styles::Style;
use dioxus_ui_system::theme::use_theme;

fn MyComponent() -> Element {
    let theme = use_theme();
    
    let style = Style::new()
        .flex()
        .flex_col()
        .items_center()
        .gap(&theme.tokens().spacing, "md")
        .bg(&theme.tokens().colors.primary)
        .p(&theme.tokens().spacing, "lg")
        .rounded(&theme.tokens().radius, "lg")
        .build();
    
    rsx! {
        div { style: "{style}", "Hello World" }
    }
}
```

## Components

### Atoms

#### Button

```rust
use dioxus_ui_system::atoms::{Button, ButtonVariant, ButtonSize};

rsx! {
    Button {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Md,
        onclick: move |_| println!("Clicked!"),
        "Click me"
    }
}
```

**Variants**: `Primary`, `Secondary`, `Ghost`, `Destructive`, `Link`

**Sizes**: `Sm`, `Md`, `Lg`, `Icon`

#### Input

```rust
use dioxus_ui_system::atoms::{Input, InputType};

let mut value = use_signal(|| String::new());

rsx! {
    Input {
        value: value(),
        input_type: InputType::Email,
        placeholder: Some("email@example.com".to_string()),
        onchange: move |v| value.set(v),
    }
}
```

#### Label

```rust
use dioxus_ui_system::atoms::{Label, TextSize, TextWeight, TextColor};

rsx! {
    Label {
        size: TextSize::H1,
        weight: TextWeight::Bold,
        color: TextColor::Primary,
        "Hello World"
    }
}
```

#### Icon

```rust
use dioxus_ui_system::atoms::{Icon, IconSize, IconColor};

rsx! {
    Icon {
        name: "check".to_string(),
        size: IconSize::Medium,
        color: IconColor::Success,
    }
}
```

Built-in icons: `check`, `x`, `plus`, `minus`, `arrow-left/right/up/down`, `chevron-*`, `menu`, `search`, `user`, `settings`, `home`, `bell`, `heart`, `star`, `trash`, `edit`, `copy`, `external-link`, `loading`, `info`, `warning`, `alert`, `error`, `moon`, `sun`

### Molecules

#### Card

```rust
use dioxus_ui_system::molecules::{Card, CardVariant, CardHeader, CardContent, CardFooter};

rsx! {
    Card {
        variant: CardVariant::Elevated,
        CardHeader {
            title: "Card Title",
            subtitle: Some("Card subtitle".to_string()),
        }
        CardContent {
            "Card content goes here"
        }
        CardFooter {
            justify: CardFooterJustify::Between,
            Button { variant: ButtonVariant::Ghost, "Cancel" }
            Button { variant: ButtonVariant::Primary, "Save" }
        }
    }
}
```

#### InputGroup

```rust
use dioxus_ui_system::molecules::InputGroup;

rsx! {
    InputGroup {
        label: "Email",
        value: email(),
        placeholder: Some("you@example.com".to_string()),
        hint: Some("We'll never share your email.".to_string()),
        error: if invalid { Some("Invalid email".to_string()) } else { None },
        required: true,
        onchange: move |v| email.set(v),
    }
}
```

#### Badge

```rust
use dioxus_ui_system::molecules::{Badge, BadgeVariant, BadgeSize};

rsx! {
    Badge {
        variant: BadgeVariant::Success,
        size: BadgeSize::Md,
        icon: Some("check".to_string()),
        "Active"
    }
}
```

### Organisms

#### Header

```rust
use dioxus_ui_system::organisms::{Header, NavItem};

let nav_items = vec![
    NavItem {
        label: "Home".to_string(),
        href: "/".to_string(),
        icon: Some("home".to_string()),
        active: true,
    },
];

rsx! {
    Header {
        brand_title: "My App",
        nav_items: nav_items,
        actions: rsx! {
            ThemeToggle {}
        }
    }
}
```

## Theming

### Theme Provider

Wrap your app with the ThemeProvider:

```rust
rsx! {
    ThemeProvider {
        initial_theme: Some(ThemeTokens::dark()),
        YourApp {}
    }
}
```

### Accessing Theme

```rust
use dioxus_ui_system::theme::use_theme;

fn MyComponent() -> Element {
    let theme = use_theme();
    let bg = theme.tokens().colors.background.to_rgba();
    
    rsx! {
        div { style: "background-color: {bg}", "Content" }
    }
}
```

### Custom Themes

```rust
use dioxus_ui_system::theme::{ThemeTokens, Color};

let mut theme = ThemeTokens::light();
theme.colors.primary = Color::new(147, 51, 234); // Purple
theme.colors.primary_foreground = Color::new(255, 255, 255);

// Or use brand helper
let brand_theme = ThemeTokens::brand(Color::new(147, 51, 234), "purple-brand");
```

## Multi-Platform Support

The library supports Web, Desktop, and Mobile via feature flags:

```toml
[dependencies]
dioxus-ui-system = { version = "0.1", default-features = false, features = ["web"] }
# or
dioxus-ui-system = { version = "0.1", default-features = false, features = ["desktop"] }
# or
dioxus-ui-system = { version = "0.1", default-features = false, features = ["mobile"] }
```

## Examples

### Live Demo & Documentation (GitHub Pages)

🚀 **View the documentation**: [https://your-username.github.io/rust-ds/](https://your-username.github.io/rust-ds/)

The documentation site includes:
- **60+ Components** with live examples and code snippets
- **Multi-page Navigation** with sidebar menu
- **Atoms**: Button, Input, Label, Icon, Checkbox, Radio, Switch, Select, TextArea, Step
- **Molecules**: Card, Badge, Alert, Avatar, Dialog, Dropdown, Tooltip, Stepper
- **Organisms**: Header, Layout, Tabs, Accordion, Cards, DataTable, Stepper Wizard
- **Themes**: 7 presets, design tokens, custom theme creation guide
- **Guides**: Quick start, styling, forms, layouts

Automatically deployed to GitHub Pages on every push to `main`.

See [GITHUB_PAGES_DEPLOYMENT.md](GITHUB_PAGES_DEPLOYMENT.md) for setup instructions.

### Run Locally

```bash
# Web
cd examples/web-csr && dx serve --platform web

# Desktop
cd examples/desktop && cargo run

# Mobile (requires setup)
cd examples/mobile && dx build --platform ios
```

## Project Structure

```
dioxus-ui-system/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Main library entry
│   ├── theme/          # Theme system
│   │   ├── tokens.rs   # Design tokens
│   │   ├── context.rs  # Theme context & provider
│   │   └── mod.rs
│   ├── styles/         # Styling system
│   │   ├── builder.rs  # Style builder
│   │   └── mod.rs
│   ├── atoms/          # Atomic design atoms
│   │   ├── button.rs
│   │   ├── input.rs
│   │   ├── label.rs
│   │   ├── icon.rs
│   │   └── mod.rs
│   ├── molecules/      # Atomic design molecules
│   │   ├── input_group.rs
│   │   ├── card.rs
│   │   ├── badge.rs
│   │   └── mod.rs
│   └── organisms/      # Atomic design organisms
│       ├── header.rs
│       ├── data_table.rs
│       └── mod.rs
└── example-app/        # Example application
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT OR Apache-2.0 license.

## Acknowledgments

- Inspired by [shadcn/ui](https://ui.shadcn.com/) and [Radix UI](https://www.radix-ui.com/)
- Built with [Dioxus](https://dioxuslabs.com/)
- Atomic Design methodology by Brad Frost

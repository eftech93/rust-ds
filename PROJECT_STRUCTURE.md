# Project Structure

## Overview

```
rust-ds/
├── dioxus-ui-system/          # Main UI library crate
├── example-app/               # Original example application
├── examples/                  # Platform-specific examples
│   ├── shared/               # Shared components for all examples
│   ├── web-csr/              # Web Client-Side Rendering
│   ├── web-ssr/              # Web Server-Side Rendering
│   ├── desktop/              # Native Desktop application
│   └── mobile/               # Native Mobile application
├── Cargo.toml                # Workspace configuration
├── README.md                 # Main documentation
└── LICENSE-*                 # License files
```

## Main Library: `dioxus-ui-system`

### Source Structure

```
dioxus-ui-system/src/
├── lib.rs                    # Library entry with prelude
├── theme/                    # Theme system
│   ├── mod.rs               # Module exports
│   ├── tokens.rs            # Design tokens (colors, spacing, typography)
│   └── context.rs           # ThemeProvider, use_theme, use_style
├── styles/                   # Styling system
│   ├── mod.rs               # Module exports, utility macros
│   └── builder.rs           # Style builder (Tailwind-like API)
├── atoms/                    # Atomic Design - Atoms
│   ├── mod.rs               # Module exports
│   ├── button.rs            # Button, IconButton, ButtonVariant, ButtonSize
│   ├── input.rs             # Input, InputType
│   ├── label.rs             # Label, TextSize, TextWeight, TextColor, Heading
│   └── icon.rs              # Icon, IconSize, IconColor (30+ built-in icons)
├── molecules/                # Atomic Design - Molecules
│   ├── mod.rs               # Module exports
│   ├── input_group.rs       # InputGroup with label, hint, error
│   ├── card.rs              # Card, CardHeader, CardContent, CardFooter
│   └── badge.rs             # Badge, BadgeVariant, StatusBadge
└── organisms/                # Atomic Design - Organisms
    ├── mod.rs               # Module exports
    ├── header.rs            # Header, NavItem, UserMenu
    └── data_table.rs        # DataTable with sorting, pagination
```

### Key Features

1. **Theme System**
   - Light/Dark/Brand theme modes
   - Type-safe design tokens
   - Reactive theme switching
   - `ThemeProvider` for context propagation

2. **Style Builder**
   - Fluent API similar to Tailwind CSS
   - Pure Rust - no CSS files
   - Inline styles for maximum performance
   - Method chaining for rapid development

3. **Components (30+ total)**
   - **Atoms**: Button, Input, Label, Icon
   - **Molecules**: Card, Badge, InputGroup
   - **Organisms**: Header, DataTable

## Examples

### Shared Components (`examples/shared`)

Reusable showcase components used by all platform examples:

- `ComponentShowcase` - Comprehensive component gallery
- Individual showcase sections for each component type
- Platform-agnostic design

### Platform Examples

| Example | Description | Run Command |
|---------|-------------|-------------|
| `web-csr` | WebAssembly client-side rendering | `./examples/run-web-csr.sh` |
| `web-ssr` | Server-side rendering with Axum | `./examples/run-web-ssr.sh` |
| `desktop` | Native desktop (Windows/Mac/Linux) | `./examples/run-desktop.sh` |
| `mobile` | iOS/Android native apps | `./examples/run-mobile-ios.sh` |

## Usage

### Basic Component Usage

```rust
use dioxus_ui_system::prelude::*;

fn MyComponent() -> Element {
    rsx! {
        ThemeProvider {
            Card {
                CardHeader { title: "Welcome" }
                CardContent {
                    Button {
                        variant: ButtonVariant::Primary,
                        "Click me"
                    }
                }
            }
        }
    }
}
```

### Style Builder

```rust
use dioxus_ui_system::styles::Style;
use dioxus_ui_system::theme::use_theme;

fn StyledComponent() -> Element {
    let theme = use_theme();
    let style = Style::new()
        .flex()
        .items_center()
        .gap(&theme.tokens().spacing, "md")
        .bg(&theme.tokens().colors.primary)
        .build();
    
    rsx! {
        div { style: "{style}", "Content" }
    }
}
```

## Development

### Build Everything

```bash
cargo build --workspace
```

### Run Tests

```bash
cargo test --workspace
```

### Run Specific Example

```bash
# Web CSR
cd examples/web-csr && dx serve --platform web

# Desktop
cd examples/desktop && cargo run

# SSR
cd examples/web-ssr && cargo run
```

## Publishing

To publish the library to crates.io:

```bash
cd dioxus-ui-system
cargo publish
```

## License

Dual-licensed under MIT OR Apache-2.0

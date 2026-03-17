# Dioxus UI Documentation

## Overview

The Dioxus UI design system includes a Storybook-like documentation interface at `/docs` route.

## Documentation Structure

```
examples/web-csr/src/docs/
├── mod.rs           # Main documentation page
├── atoms.rs         # Atom component docs
├── molecules.rs     # Molecule component docs  
├── organisms.rs     # Organism component docs
├── themes.rs        # Theme system docs
└── guides.rs        # Usage guides
```

## Features

### 📚 Component Documentation
- **Atoms**: Button, Input, Label, Icon, Checkbox, Radio, Switch, Select, TextArea, Step
- **Molecules**: Card, Badge, Alert, Avatar, Dialog, Dropdown, Popover, Tooltip, Separator, Skeleton, Stepper
- **Organisms**: Header, Layout, Tabs, Accordion, Cards, DataTable, Stepper Wizard

### 🎨 Theme System
- 7 preset themes (Light, Dark, Rose, Blue, Green, Violet, Orange)
- Custom theme creation guide
- Design tokens reference

### 📖 Usage Guides
- Quick start guide
- Styling guide
- Form building
- Page layouts
- Responsive design

## Accessing Documentation

### Local Development
```bash
cd examples/web-csr
dx serve --platform web
# Navigate to http://localhost:8080
# Click "Documentation" tab
```

### Production (GitHub Pages)
The documentation is deployed automatically with GitHub Pages:
```
https://<username>.github.io/rust-ds/
```

## Customizing Documentation

### Adding Component Examples

Edit the respective docs file in `examples/web-csr/src/docs/`:

```rust
// In atoms.rs
ComponentSection {
    title: "Button",
    description: "Interactive button component",
    
    // Live examples
    Button { variant: ButtonVariant::Primary, "Click Me" }
    
    // Code example
    CodeExample {
        code: "Button {{ variant: ButtonVariant::Primary, \"Click Me\" }}".to_string()
    }
}
```

### Adding New Documentation Pages

1. Create a new section in `DocsSidebar` in `mod.rs`
2. Add the route handler in the `match` statement
3. Create the page component

## Note on CSS Colors

Due to Dioxus 0.6's format string handling, hex colors (`#rrggbb`) in style strings should be converted to:
- Named colors: `white`, `black`, `red`, etc.
- RGB format: `rgb(255,255,255)`

Example:
```rust
// Instead of:
style: "background: #f8fafc;"

// Use:
style: "background: rgb(248,250,252);"
// or
style: "background: white;"
```

## Future Enhancements

- [ ] Interactive component playground
- [ ] Copy-to-clipboard for code examples
- [ ] Search functionality
- [ ] Dark mode for documentation
- [ ] Mobile-responsive documentation layout
- [ ] Component prop tables with types
- [ ] Version selector for docs

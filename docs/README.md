# Dioxus UI Documentation Site

A comprehensive documentation site for the Dioxus UI System with multi-page navigation, component examples, and theme building guides.

## Structure

- **Atoms** - Basic building blocks (Button, Input, Label, etc.)
- **Molecules** - Component combinations (Card, Dialog, Badge, etc.)
- **Organisms** - Complex layouts (Header, Tabs, Accordion, etc.)
- **Themes** - Design tokens and customization
- **Guides** - Usage tutorials and best practices

## Development

```bash
# Run development server
dx serve --platform web

# Build for production
dx build --platform web --release
```

## Deployment

The site is configured to deploy to GitHub Pages at `/rust-ds/`.

To deploy:

1. Build the release version:
   ```bash
   dx build --platform web --release
   ```

2. Copy the output from `target/dx/dioxus-ui-docs/release/web/public/` to your GitHub Pages branch.

## Features

- Multi-page navigation with sidebar
- Interactive component examples
- Live theme switching
- Props documentation tables
- Code examples with syntax highlighting

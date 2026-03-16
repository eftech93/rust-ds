# Dioxus UI System Examples

This directory contains example applications demonstrating the Dioxus UI System across different platforms.

## Examples Overview

| Example | Platform | Description | Run Command |
|---------|----------|-------------|-------------|
| `web-csr` | Web (WASM) | Client-side rendering with WebAssembly | **`dx serve --platform web`** ⚠️ |
| `web-ssr` | Web (Server) | Server-side rendering with Axum | `cargo run` (tries ports 3000-3003, 8080) |
| `desktop` | Desktop | Native desktop app (Windows/Mac/Linux) | `cargo run` |
| `mobile` | Mobile | iOS/Android native apps | `dx bundle --platform ios/android` |

⚠️ **Note:** Web examples require `dx` (Dioxus CLI), not `cargo run`.

## Shared Components

The `shared` crate contains reusable components that are used across all examples:

- `ComponentShowcase` - Comprehensive UI component gallery
- `AppHeader` - Navigation header component
- Individual showcase components for each component category

## Running Examples

### Web (Client-Side Rendering)

```bash
cd examples/web-csr

# Development server with hot reload
dx serve --platform web

# Or with explicit hot reload
dx serve --platform web --hot-reload

# Build for production
dx build --platform web --release
```

### Web (Server-Side Rendering)

```bash
cd examples/web-ssr

# Run the Axum server
cargo run

# Open http://localhost:3000 in your browser
```

### Desktop

```bash
cd examples/desktop

# Run in development mode
cargo run

# Build for production
cargo build --release

# Create app bundle (macOS)
cargo bundle --release
```

### Mobile

#### iOS

```bash
cd examples/mobile

# Build for iOS simulator
dx bundle --platform ios

# Or run with hot reload (experimental)
dx serve --platform ios
```

#### Android

```bash
cd examples/mobile

# Build for Android
dx bundle --platform android

# Or run with hot reload (experimental)
dx serve --platform android
```

## Project Structure

```
examples/
├── shared/          # Shared components library
│   ├── src/
│   │   └── lib.rs   # ComponentShowcase and helpers
│   └── Cargo.toml
├── web-csr/         # Web Client-Side Rendering
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── web-ssr/         # Web Server-Side Rendering
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── desktop/         # Native Desktop App
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
└── mobile/          # Native Mobile App
    ├── src/
    │   └── main.rs
    └── Cargo.toml
```

## ComponentShowcase

The main `ComponentShowcase` component displays all available UI components:

- **Buttons** - All variants, sizes, and states
- **Form Inputs** - Input fields with validation
- **Badges** - Status indicators and labels
- **Cards** - Different card variants
- **Icons** - 30+ built-in icons
- **Typography** - Text styles and formatting
- **Interactive Demo** - State management example
- **Theme System** - Light/dark mode toggle

## Platform-Specific Features

### Web CSR
- WebAssembly compilation
- Hot reload support
- SEO-friendly structure

### Web SSR
- Server-side rendering with Axum
- Fast initial page load
- Hydration support

### Desktop
- Native window controls
- Menu bar (can be disabled)
- System tray support (optional)

### Mobile
- Safe area handling (notch, home indicator)
- Touch-optimized interface
- Platform-specific navigation

## Notes

- All examples use the same `ComponentShowcase` from the `shared` crate
- Platform-specific banners indicate which platform is running
- The theme system works consistently across all platforms
- Responsive design adapts to different screen sizes

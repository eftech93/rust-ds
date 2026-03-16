# iOS ThemeContext Issue

## Problem

On iOS, there's a known issue where `ThemeContext` cannot be found when using components from `dioxus-ui-system` that call `use_theme()` internally (like `Label`, `Heading`, `Badge`, `Button`, etc.).

Error message:
```
Could not find context dioxus_ui_system::theme::context::ThemeContext
```

## Workaround

The current workaround is to use **plain HTML** instead of the themed components from the library.

### Working Example (Plain HTML)

```rust
#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "font-family: system-ui;",
            
            // ✅ Works: Plain HTML
            h1 { "Hello" }
            p { "World" }
            button { "Click me" }
        }
    }
}
```

### NOT Working (Theme-dependent components)

```rust
#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            // ❌ Crashes on iOS: Cannot find ThemeContext
            Label { "Hello" }
            Heading { level: HeadingLevel::H1, "Title" }
            Button { "Click" }
        }
    }
}
```

## Root Cause

This appears to be a platform-specific issue with Dioxus on iOS where context providers are not propagating correctly. The same code works on:
- ✅ Web (CSR/SSR)
- ✅ Desktop
- ❌ iOS
- ❓ Android (untested)

## Building the Working Version

```bash
cd examples/mobile
dx bundle --platform ios
./../run-ios-sim.sh
```

## Future Fix

This needs to be investigated further. Possible causes:
1. Dioxus mobile context propagation bug
2. Different rendering order on mobile
3. Signal/context initialization timing issue

For now, use plain HTML with inline styles for iOS apps.

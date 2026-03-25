# Component Customization Roadmap

## Completed ✅

### 1. Optional Theme Persistence
- Added `persist_theme: bool` prop to `ThemeProvider`
- Defaults to `false` (no persistence)
- When enabled, saves/loads from localStorage

```rust
// No persistence (default)
ThemeProvider { MyApp {} }

// With persistence
ThemeProvider { persist_theme: true, MyApp {} }
```

### 2. Global Configuration System
- Created `config` module in `dioxus-ui-system`
- Added `Config` and `ConfigBuilder` structs
- Global config accessible via `global_config()`

```rust
ComponentConfig::builder()
    .button_size("lg")
    .card_padding("24px")
    .transitions(true)
    .build();
```

### 3. Component Analysis
- Analyzed docs-app components
- Identified reusable documentation components
- Created migration plan

## In Progress 🔄

### Component Style Override System
All components already support:
- `style: Option<String>` - Inline CSS
- `class: Option<String>` - CSS class names
- Variants (ButtonVariant, CardVariant, etc.)
- Sizes (ButtonSize, InputSize, etc.)

## Planned 📋

### 1. Move Docs Components to Library
Create `dioxus-ui-system/src/docs/` with:
- `DocPage` - Documentation page layout
- `Section` - Content section
- `ExampleBox` - Component example container
- `CodeBlock` - Code display
- `ThemePreview` - Theme live preview

### 2. Enhanced Builder Pattern (Future)
Investigate builder-style API:

```rust
Button::builder()
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Lg)
    .style("border-radius: 20px;")
    .onclick(|_| {})
    .build()
```

### 3. CSS Custom Properties Integration
Better integration with CSS custom properties for:
- Dynamic theme switching
- Component-level overrides
- Runtime customization

## Component Customization Checklist

| Component | style Prop | class Prop | Variants | Sizes | Config Support |
|-----------|-----------|-----------|----------|-------|----------------|
| Button | ✅ | ✅ | ✅ | ✅ | ✅ via Config |
| Input | ✅ | ✅ | ❌ | ✅ | ✅ via Config |
| Card | ✅ | ✅ | ✅ | ❌ | ✅ via Config |
| Badge | ✅ | ✅ | ✅ | ✅ | ❌ |
| Alert | ✅ | ✅ | ✅ | ❌ | ❌ |
| Avatar | ✅ | ✅ | ❌ | ✅ | ❌ |
| Dialog | ✅ | ✅ | ❌ | ❌ | ❌ |
| ... | ... | ... | ... | ... | ... |

## Usage Patterns Summary

### Pattern 1: Theme-Based (Global)
```rust
ThemeProvider { persist_theme: true,
    App {}
}
```

### Pattern 2: Global Config (App-wide defaults)
```rust
fn main() {
    ComponentConfig::builder()
        .button_size("lg")
        .build();
    dioxus::launch(App);
}
```

### Pattern 3: Variants (Semantic styles)
```rust
Button { variant: ButtonVariant::Primary, "Click me" }
```

### Pattern 4: Inline Styles (One-off)
```rust
Button {
    style: "border-radius: 20px;",
    "Custom"
}
```

### Pattern 5: CSS Classes (Complex)
```rust
Button {
    class: "my-custom-button",
    "Styled"
}
```

## Next Steps

1. **Immediate**: Move docs components to library
2. **Short-term**: Ensure all components have consistent `style`/`class` support
3. **Medium-term**: Implement builder pattern experimentally
4. **Long-term**: Full CSS-in-Rust integration

## Migration Guide for Users

### From v0.0.4 to v0.0.5

No breaking changes. New features are additive:

1. Theme persistence is now opt-in (`persist_theme: true`)
2. Global config is available but optional
3. All existing code continues to work

To enable new features:
```rust
// In your main()
ComponentConfig::builder()
    .button_size("lg")  // Your preferred default
    .build();

// In your app
ThemeProvider { persist_theme: true,  // Enable persistence
    MyApp {}
}
```

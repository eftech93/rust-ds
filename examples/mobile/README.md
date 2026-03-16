# Mobile Example

This example demonstrates the Dioxus UI System running as a native mobile app with full component showcase.

## Features

- **Welcome Page** - Landing screen with app intro
- **Components Page** - Full component showcase similar to desktop/web
- **Navigation** - Navigate between pages
- **Theme System** - Full light/dark theme support
- **Responsive Design** - Works on all screen sizes

## Screens

### 1. Welcome Page
- App logo and title
- Description and feature badges
- "Get Started" button

### 2. Components Page
- Buttons (variants, sizes)
- Input fields with validation
- Badges (status indicators)
- Cards (elevated, outlined)
- Icons (30+ built-in icons)
- Typography (headings, labels)
- Interactive counter demo

## Running

### iOS

```bash
cd examples/mobile
dx bundle --platform ios
./../run-ios-sim.sh
```

### Android

```bash
cd examples/mobile
dx bundle --platform android
```

## Navigation

- Tap "Get Started" to view components
- Tap "← Back" to return to welcome page

## Architecture

```
App
└── ThemeProvider
    └── MobileApp
        ├── WelcomePage (initial)
        └── ComponentsPage (after navigation)
            ├── ButtonShowcase
            ├── InputShowcase
            ├── BadgeShowcase
            ├── CardShowcase
            ├── IconShowcase
            ├── TypographyShowcase
            └── CounterDemo
```

## Notes

- Uses `use_signal` for navigation state
- All components from `dioxus-ui-system` work on mobile
- Safe area handling for notch and home indicator

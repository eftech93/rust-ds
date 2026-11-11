# Examples

Example apps for `dioxus-ui-system` on each supported platform. They all use the local
crate via a path dependency and share their showcase UI through `examples/shared`.

| Crate | Directory | Platform | Run (from the repo root) |
|-------|-----------|----------|-----|
| `example-shared`  | `shared/`  | library | used by the others |
| `example-web-csr` | `web-csr/` | browser (WASM) | `scripts/dev.sh web` |
| `example-web-ssr` | `web-ssr/` | server-rendered HTML (Axum) | `scripts/dev.sh ssr` |
| `example-desktop` | `desktop/` | Windows / macOS / Linux | `scripts/dev.sh desktop` |
| `example-mobile`  | `mobile/`  | iOS / Android | `scripts/dev.sh ios` / `scripts/dev.sh android` |

Prerequisites: Rust stable, `rustup target add wasm32-unknown-unknown`, and the Dioxus CLI
0.7 (`cargo binstall dioxus-cli@0.7`) for the web and mobile examples. See
[CONTRIBUTING.md](../CONTRIBUTING.md) for the full setup.

`scripts/dev.sh doctor` checks your setup. The `scripts/dev.sh` commands wrap the plain
`dx`/`cargo` commands shown below.

## shared

`ComponentShowcase` / `ComponentShowcaseInner` (gallery of buttons, inputs, badges, cards,
icons, typography, an interactive counter and the theme switcher), `LayoutShowcase` /
`LayoutShowcaseInner`, and `AppHeader`. The `*Inner` variants expect the caller to provide
the `ThemeProvider`.

## web-csr

Client-side rendered app compiled to WebAssembly.

```bash
cd examples/web-csr
dx serve --platform web                 # http://localhost:8080, hot reload
dx serve --platform web --port 3000     # if 8080 is taken
dx build --platform web --release       # production build
```

`cargo run` does not work here: the crate targets WASM and must be built and served by `dx`.

## web-ssr

Renders the showcase to HTML on the server with `dioxus::ssr::render_element()` and serves
it with Axum. No client-side interactivity.

```bash
cd examples/web-ssr
cargo run
```

The server tries ports 3000, 3001, 3002, 3003 and 8080 in order and prints the URL it
bound to.

## desktop

Native window (1400x900, menu bar disabled) showing the shared showcase.

```bash
cd examples/desktop
cargo run              # or: cargo run --release
```

On Linux you need the WebKitGTK dev packages:
`sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev libayatana-appindicator3-dev`.

## mobile

Native app with a welcome page, a components page, a layouts page and a theme toggle, using
signals for navigation.

```bash
cd examples/mobile
dx serve --platform ios        # build, install and run on the simulator
dx serve --platform android    # build, install and run on an emulator/device
dx bundle --platform ios       # bundle only (also: --platform android, --release)
```

Simulator helpers: `scripts/dev.sh ios sim` boots a simulator, `ios logs` streams the app's
logs, `ios status` shows whether the app is installed and lists recent crash reports, and
`ios reset` erases all simulators.

### iOS setup (macOS)

1. Install Xcode from the App Store, then:
   ```bash
   xcode-select --install
   sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
   sudo xcodebuild -license accept
   ```
2. Install an iOS simulator runtime: Xcode → Settings → Platforms → iOS
   (or `xcrun simctl runtime add ios`).
3. Add the Rust targets:
   ```bash
   rustup target add aarch64-apple-ios aarch64-apple-ios-sim
   ```

### Android setup

1. Install Android Studio with the SDK, platform tools and NDK.
2. Set environment variables (e.g. in `~/.zshrc`):
   ```bash
   export ANDROID_HOME=$HOME/Library/Android/sdk     # Linux: $HOME/Android/Sdk
   export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/<version>
   export PATH=$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools
   ```
3. Add the Rust targets:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
   ```

### Troubleshooting

| Problem | Fix |
|---------|-----|
| `dx: command not found` | Add `~/.cargo/bin` to `PATH`, or reinstall the CLI |
| `SDK "iphonesimulator" cannot be located` | Install Xcode and a simulator runtime, then `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`; verify with `xcrun --show-sdk-path --sdk iphonesimulator` |
| `tool 'xcodebuild' requires Xcode` | `sudo xcode-select --reset`, then switch to the Xcode path as above |
| Xcode installed under another name | Find it with `mdfind "kMDItemCFBundleIdentifier == 'com.apple.dt.Xcode'"` |
| Android NDK not found | Set `ANDROID_NDK_HOME` to the installed NDK version |
| Missing Rust targets | `rustup target list --installed` and add the ones listed above |
| App logs | `scripts/dev.sh ios logs` streams simulator logs |

Earlier versions of the library crashed on iOS with "Could not find context ThemeContext"
when themed components rendered. Since 0.0.8 `use_theme()` falls back to a light theme instead of
panicking, but keep the app wrapped in a `ThemeProvider` so theme changes apply app-wide.

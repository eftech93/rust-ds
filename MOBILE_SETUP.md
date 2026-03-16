# Mobile Setup Guide

## Install Dioxus CLI

The `dx` command is part of the Dioxus CLI. Install it with:

```bash
cargo install dioxus-cli@0.6.3
```

Or install the latest version:
```bash
cargo install dioxus-cli
```

Verify installation:
```bash
dx --version
```

## Android Setup

### Prerequisites

1. **Install Android Studio**
   - Download from: https://developer.android.com/studio
   - Install Android SDK and NDK

2. **Set Environment Variables**
   Add to your `~/.bashrc`, `~/.zshrc`, or `~/.bash_profile`:

   ```bash
   export ANDROID_HOME=$HOME/Library/Android/sdk  # macOS
   # export ANDROID_HOME=$HOME/Android/Sdk        # Linux
   
   export PATH=$PATH:$ANDROID_HOME/emulator
   export PATH=$PATH:$ANDROID_HOME/platform-tools
   export PATH=$PATH:$ANDROID_HOME/tools
   export PATH=$PATH:$ANDROID_HOME/tools/bin
   ```

3. **Install Required SDK Components**
   ```bash
   sdkmanager "platform-tools" "platforms;android-33" "build-tools;33.0.0" "ndk;25.2.9519653"
   ```

4. **Install Rust Android Targets**
   ```bash
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add x86_64-linux-android
   ```

## Building the Mobile App

### Using dx (Recommended)

```bash
cd examples/mobile

# Build for Android
dx bundle --platform android

# Or run with hot reload (experimental)
dx serve --platform android
```

### Using Cargo (Without dx)

If you don't want to use `dx`, you can use `cargo apk`:

```bash
# Install cargo-apk
cargo install cargo-apk

# Build APK
cargo apk build --release
```

But this requires additional setup in Cargo.toml.

## iOS Setup (macOS only)

### Prerequisites

1. **Xcode** - Install from App Store

2. **Install iOS targets**
   ```bash
   rustup target add aarch64-apple-ios
   rustup target add aarch64-apple-ios-sim
   ```

### Building for iOS

```bash
cd examples/mobile

# Build for iOS
dx bundle --platform ios

# Or run on simulator
dx serve --platform ios
```

## Troubleshooting

### "dx command not found"
```bash
# Make sure cargo bin is in your PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Reinstall if needed
cargo install dioxus-cli --force
```

### Android NDK not found
```bash
# Set NDK path explicitly
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653
```

### Build fails on macOS
Make sure you have Xcode Command Line Tools:
```bash
xcode-select --install
```

## Alternative: Manual Build Without dx

If you prefer not to use `dx`, you can build manually:

```bash
cd examples/mobile

# Build for Android (debug)
cargo build --target aarch64-linux-android

# Note: For actual APK generation, dx or cargo-apk is required
```

## Quick Start Script

I've created a helper script that checks for `dx` and provides instructions:

```bash
./examples/check-mobile-setup.sh
```

Run this to verify your mobile development environment.

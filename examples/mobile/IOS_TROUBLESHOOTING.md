# iOS Build Troubleshooting

## Error: SDK "iphonesimulator" cannot be located

This error means the iOS Simulator SDK is not installed or Xcode is not properly configured.

## Solution

### 1. Install Xcode

Download and install Xcode from the Mac App Store:
https://apps.apple.com/us/app/xcode/id497799835

### 2. Install Command Line Tools

```bash
xcode-select --install
```

### 3. Accept Xcode License

```bash
sudo xcodebuild -license accept
```

### 4. Select Xcode Command Line Tools

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

### 5. Install iOS Simulator

Open Xcode:
1. Go to **Xcode** → **Preferences** → **Platforms**
2. Click **"+"** to add a platform
3. Select **iOS** and install the latest version

Or use command line:
```bash
# List available simulators
xcrun simctl list devices

# Install iOS runtime
xcrun simctl runtime add ios
```

### 6. Verify Installation

```bash
# Check that SDK is found
xcrun --show-sdk-path --sdk iphonesimulator

# Should output something like:
# /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator17.0.sdk

# Check Xcode version
xcodebuild -version
```

### 7. Install Rust iOS Targets

```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
```

## Build Again

After completing the above steps:

```bash
cd examples/mobile
dx bundle --platform ios
```

## Alternative: Build Without Simulator (Device Only)

If you only want to build for a physical device:

```bash
cd examples/mobile
dx bundle --platform ios --release
```

## Common Issues

### "xcode-select: error: tool 'xcodebuild' requires Xcode"

Xcode is not installed or the path is wrong. Run:
```bash
sudo xcode-select --reset
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

### "No such file or directory: '/Applications/Xcode.app'"

Xcode is installed with a different name. Find it:
```bash
mdfind "kMDItemCFBundleIdentifier == 'com.apple.dt.Xcode'"
```

Then set the correct path:
```bash
sudo xcode-select --switch /path/to/Xcode.app/Contents/Developer
```

### Rust targets not installed

```bash
rustup target list | grep ios
```

If not installed:
```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

## Quick Check Script

Run this to check your iOS setup:

```bash
#!/bin/bash
echo "Checking iOS development environment..."
echo ""

# Check Xcode
if xcodebuild -version &>/dev/null; then
    echo "✅ Xcode: $(xcodebuild -version | head -1)"
else
    echo "❌ Xcode not found"
    echo "   Install from App Store: https://apps.apple.com/us/app/xcode/id497799835"
fi

# Check SDK
if xcrun --show-sdk-path --sdk iphonesimulator &>/dev/null; then
    echo "✅ iOS Simulator SDK: $(xcrun --show-sdk-path --sdk iphonesimulator)"
else
    echo "❌ iOS Simulator SDK not found"
    echo "   Open Xcode → Preferences → Platforms → Install iOS"
fi

# Check Rust targets
echo ""
echo "Rust iOS targets:"
rustup target list --installed | grep ios || echo "   None installed (run: rustup target add aarch64-apple-ios aarch64-apple-ios-sim)"
```

Save this as `check-ios.sh` and run it to verify your setup.

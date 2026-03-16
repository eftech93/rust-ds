#!/bin/bash
# Check mobile development environment setup

echo "========================================="
echo "Mobile Development Environment Check"
echo "========================================="
echo ""

# Check for dx command
echo -n "Checking for dx (Dioxus CLI)... "
if command -v dx &> /dev/null; then
    echo "✅ Found"
    dx --version
else
    echo "❌ Not found"
    echo ""
    echo "To install dx, run:"
    echo "  cargo install dioxus-cli@0.6.3"
    echo ""
fi
echo ""

# Check for Android SDK
echo -n "Checking for ANDROID_HOME... "
if [ -n "$ANDROID_HOME" ]; then
    echo "✅ Set to: $ANDROID_HOME"
else
    echo "❌ Not set"
    echo "  Please set ANDROID_HOME to your Android SDK path"
    echo "  Example: export ANDROID_HOME=\$HOME/Library/Android/sdk"
fi
echo ""

# Check for Rust Android targets
echo "Checking Rust Android targets..."
TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "x86_64-linux-android")
for target in "${TARGETS[@]}"; do
    echo -n "  $target... "
    if rustup target list --installed | grep -q "$target"; then
        echo "✅"
    else
        echo "❌ (run: rustup target add $target)"
    fi
done
echo ""

# Check for cargo-apk (alternative to dx)
echo -n "Checking for cargo-apk (alternative)... "
if command -v cargo-apk &> /dev/null; then
    echo "✅ Found"
else
    echo "❌ Not found (optional)"
    echo "  Install with: cargo install cargo-apk"
fi
echo ""

# Platform specific checks
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Platform: macOS"
    echo ""
    echo -n "Checking for Xcode... "
    if xcode-select -p &> /dev/null; then
        echo "✅ Found"
    else
        echo "❌ Not found"
        echo "  Install Xcode from the App Store"
    fi
    echo ""
    
    echo "Checking Rust iOS targets..."
    IOS_TARGETS=("aarch64-apple-ios" "aarch64-apple-ios-sim")
    for target in "${IOS_TARGETS[@]}"; do
        echo -n "  $target... "
        if rustup target list --installed | grep -q "$target"; then
            echo "✅"
        else
            echo "❌ (run: rustup target add $target)"
        fi
    done
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Platform: Linux"
    echo "  Note: iOS development is not supported on Linux"
else
    echo "Platform: Unknown ($OSTYPE)"
fi

echo ""
echo "========================================="
echo "Summary"
echo "========================================="
echo ""

if ! command -v dx &> /dev/null; then
    echo "⚠️  dx is not installed. Install it with:"
    echo "   cargo install dioxus-cli@0.6.3"
    echo ""
fi

echo "To build the mobile app:"
echo "  cd examples/mobile"
echo ""
echo "For Android:"
echo "  dx bundle --platform android"
echo ""

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "For iOS:"
    echo "  dx bundle --platform ios"
    echo ""
fi

echo "For more details, see MOBILE_SETUP.md"
echo ""

#!/bin/bash
# Run iOS app on simulator

APP_PATH="$(dirname "$0")/../target/dx/example-mobile/debug/ios/ExampleMobile.app"
BUNDLE_ID="com.example.ExampleMobile"

echo "========================================="
echo "iOS Simulator Launcher"
echo "========================================="
echo ""

# Check if app exists
if [ ! -d "$APP_PATH" ]; then
    echo "❌ App not found at: $APP_PATH"
    echo ""
    echo "Build first with:"
    echo "   cd examples/mobile && dx bundle --platform ios"
    exit 1
fi

echo "✅ App found: $APP_PATH"
echo ""

# List available simulators
echo "Available iOS Simulators:"
xcrun simctl list devices 2>/dev/null | grep -E "(iPhone|iPad)" | grep -v "unavailable" | head -10
echo ""

# Find first available iPhone simulator
DEVICE=$(xcrun simctl list devices 2>/dev/null | grep "iPhone" | grep -v "unavailable" | head -1 | sed 's/^ *//' | cut -d'(' -f1 | sed 's/ *$//')

if [ -z "$DEVICE" ]; then
    echo "❌ No iPhone simulators found!"
    echo ""
    echo "Create one with:"
    echo "   ./examples/create-ios-simulator.sh"
    exit 1
fi

echo "Using device: $DEVICE"
echo ""

# Boot the device
if ! xcrun simctl list devices 2>/dev/null | grep "$DEVICE" | grep -q "Booted"; then
    echo "📱 Booting simulator..."
    xcrun simctl boot "$DEVICE" 2>/dev/null
    if [ $? -ne 0 ]; then
        echo "❌ Failed to boot simulator"
        echo "Try using the device UUID instead:"
        xcrun simctl list devices | grep "$DEVICE"
        exit 1
    fi
fi

# Open Simulator app
if ! pgrep -q "Simulator" 2>/dev/null; then
    echo "🖥️  Opening Simulator app..."
    open -a Simulator
    sleep 2
fi

echo "📲 Installing app..."
xcrun simctl install booted "$APP_PATH"
if [ $? -ne 0 ]; then
    echo "❌ Failed to install app"
    exit 1
fi

echo "🚀 Launching app..."
xcrun simctl launch booted "$BUNDLE_ID"
if [ $? -ne 0 ]; then
    echo "❌ Failed to launch app"
    exit 1
fi

echo ""
echo "========================================="
echo "✅ App is now running on iOS Simulator!"
echo "========================================="
echo ""
echo "Device: $DEVICE"
echo "Bundle ID: $BUNDLE_ID"
echo ""
echo "To view logs:"
echo "   xcrun simctl spawn booted log stream --predicate 'process == \"ExampleMobile\"'"

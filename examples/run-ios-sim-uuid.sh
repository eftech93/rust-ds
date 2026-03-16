#!/bin/bash
# Run iOS app using UUID (more reliable)

APP_PATH="$(dirname "$0")/../target/dx/example-mobile/debug/ios/ExampleMobile.app"
BUNDLE_ID="com.example.ExampleMobile"

echo "========================================="
echo "iOS Simulator Launcher (UUID Method)"
echo "========================================="
echo ""

# Check if app exists
if [ ! -d "$APP_PATH" ]; then
    echo "❌ App not found. Build first:"
    echo "   cd examples/mobile && dx bundle --platform ios"
    exit 1
fi

# Get first available iPhone UUID
UUID=$(xcrun simctl list devices 2>/dev/null | grep "iPhone" | grep -v "unavailable" | head -1 | grep -oE "[A-F0-9]{8}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{12}")

if [ -z "$UUID" ]; then
    echo "❌ No iPhone simulators found"
    exit 1
fi

echo "Using simulator UUID: $UUID"
echo ""

# Shutdown first (helps with error 405)
echo "1. Shutting down simulator (if running)..."
xcrun simctl shutdown "$UUID" 2>/dev/null
sleep 1

# Boot the device
echo "2. Booting simulator..."
xcrun simctl boot "$UUID"
if [ $? -ne 0 ]; then
    echo "❌ Failed to boot"
    exit 1
fi

# Open Simulator app
echo "3. Opening Simulator app..."
open -a Simulator
sleep 2

# Install app
echo "4. Installing app..."
xcrun simctl install "$UUID" "$APP_PATH"
if [ $? -ne 0 ]; then
    echo "❌ Install failed. Trying to fix..."
    echo "   Run: ./examples/fix-ios-simulator.sh"
    exit 1
fi

# Launch app
echo "5. Launching app..."
xcrun simctl launch "$UUID" "$BUNDLE_ID"
if [ $? -ne 0 ]; then
    echo "❌ Launch failed"
    exit 1
fi

echo ""
echo "✅ App is running!"

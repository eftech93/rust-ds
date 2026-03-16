#!/bin/bash
# Debug iOS app with full logging

echo "========================================="
echo "iOS Debug Tool"
echo "========================================="
echo ""

# Check if simulator is booted
BOOTED=$(xcrun simctl list devices 2>/dev/null | grep "Booted")
if [ -z "$BOOTED" ]; then
    echo "❌ No booted simulator"
    exit 1
fi

UUID=$(echo "$BOOTED" | grep -oE "[A-F0-9]{8}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{12}" | head -1)
echo "Simulator UUID: $UUID"
echo ""

# Check if app is installed
echo "=== Installed Apps ==="
xcrun simctl listapps "$UUID" 2>/dev/null | grep -E "bundleID|bundlePath" | head -10
echo ""

# Get app container path
echo "=== App Container ==="
APP_PATH=$(xcrun simctl get_app_container "$UUID" com.example.ExampleMobile 2>/dev/null)
if [ -n "$APP_PATH" ]; then
    echo "App container: $APP_PATH"
    ls -la "$APP_PATH" 2>/dev/null | head -10
else
    echo "❌ App not installed"
fi
echo ""

# View crash logs
echo "=== Recent Crash Logs ==="
CRASH_DIR="$HOME/Library/Logs/DiagnosticReports"
if [ -d "$CRASH_DIR" ]; then
    ls -lt "$CRASH_DIR" | grep -i "ExampleMobile\|dioxus" | head -5
fi

# Simulator logs
echo ""
echo "=== Booting Simulator Log ==="
xcrun simctl bootlog "$UUID" 2>/dev/null | tail -20

echo ""
echo "========================================="
echo "To view live logs, run:"
echo "   ./examples/view-ios-logs.sh"
echo "========================================="

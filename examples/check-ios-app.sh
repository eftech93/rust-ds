#!/bin/bash
# Check if the iOS app is running properly

echo "========================================="
echo "iOS App Status Check"
echo "========================================="
echo ""

# Get booted simulator
UUID=$(xcrun simctl list devices 2>/dev/null | grep "Booted" | grep -oE "[A-F0-9]{8}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{12}" | head -1)

if [ -z "$UUID" ]; then
    echo "❌ No booted simulator found"
    exit 1
fi

echo "Simulator UUID: $UUID"
echo ""

# Check if app is installed
echo "=== App Installation Status ==="
APP_INFO=$(xcrun simctl listapps "$UUID" 2>/dev/null | grep -A5 "com.example.ExampleMobile")
if [ -n "$APP_INFO" ]; then
    echo "✅ App is installed"
    echo "$APP_INFO"
else
    echo "❌ App is NOT installed"
fi
echo ""

# Check if app is running
echo "=== Running Processes ==="
RUNNING=$(xcrun simctl spawn "$UUID" ps aux 2>/dev/null | grep -i "examplemobile\|dioxus" | grep -v grep)
if [ -n "$RUNNING" ]; then
    echo "✅ App process found:"
    echo "$RUNNING"
else
    echo "⚠️  App process not found (may not be running)"
fi
echo ""

# Check recent system logs for our app
echo "=== Recent App Logs ==="
xcrun simctl spawn "$UUID" log show --last 1m --predicate 'process == "ExampleMobile"' 2>/dev/null | tail -20
echo ""

# Take a screenshot
echo "=== Taking Screenshot ==="
xcrun simctl io "$UUID" screenshot "/tmp/ios_screenshot.png" 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ Screenshot saved to: /tmp/ios_screenshot.png"
    echo "   Open it with: open /tmp/ios_screenshot.png"
else
    echo "❌ Failed to take screenshot"
fi
echo ""

# Open the simulator app to see it
echo "========================================="
echo "Opening Simulator app..."
open -a Simulator
echo ""
echo "If the app crashed, check:"
echo "   Console.app → Crash Reports"
echo "   Or run: ./examples/view-ios-logs.sh"

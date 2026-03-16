#!/bin/bash
# View iOS Simulator logs for the app

echo "========================================="
echo "iOS Simulator Logs"
echo "========================================="
echo ""

# Get the booted simulator UUID
UUID=$(xcrun simctl list devices 2>/dev/null | grep "Booted" | grep -oE "[A-F0-9]{8}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{4}-[A-F0-9]{12}" | head -1)

if [ -z "$UUID" ]; then
    echo "❌ No booted simulator found"
    echo "   Run: ./examples/run-ios-sim.sh"
    exit 1
fi

echo "Viewing logs for simulator: $UUID"
echo ""
echo "Press Ctrl+C to stop"
echo ""

# Option 1: System log (most detailed)
echo "=== System Log (with crash details) ==="
xcrun simctl spawn "$UUID" log stream --level debug --predicate 'process == "ExampleMobile" OR eventMessage CONTAINS "panic" OR eventMessage CONTAINS "error"'

# Alternative logs if the above doesn't work:
# echo "=== Console.app style logs ==="
# xcrun simctl logverbose "$UUID"

# echo "=== App specific logs ==="  
# xcrun simctl launch --console "$UUID" com.example.ExampleMobile

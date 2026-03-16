#!/bin/bash
# Fix iOS Simulator issues (error 405)

echo "========================================="
echo "iOS Simulator Troubleshooter"
echo "========================================="
echo ""

# Check if simulator is booted
echo "1. Checking simulator state..."
BOOTED=$(xcrun simctl list devices 2>/dev/null | grep "Booted")
if [ -n "$BOOTED" ]; then
    echo "   Found booted simulator:"
    echo "   $BOOTED"
    echo ""
    
    echo "2. Shutting down all simulators..."
    xcrun simctl shutdown all 2>/dev/null
    sleep 2
    echo "   ✅ Simulators shut down"
    echo ""
fi

echo "3. Erasing simulator data (keeps devices)..."
# Note: This erases app data but keeps the simulators
xcrun simctl erase all 2>/dev/null
echo "   ✅ Simulator data erased"
echo ""

echo "4. Killing Simulator app..."
killall "Simulator" 2>/dev/null
killall "iOS Simulator" 2>/dev/null
sleep 1
echo "   ✅ Simulator app killed"
echo ""

echo "5. Restarting CoreSimulator service..."
sudo killall -9 com.apple.CoreSimulator.CoreSimulatorService 2>/dev/null
sleep 2
echo "   ✅ CoreSimulator service restarted"
echo ""

echo "========================================="
echo "✅ Troubleshooting complete!"
echo "========================================="
echo ""
echo "Now try running your app again:"
echo "   ./examples/run-ios-sim.sh"
echo ""

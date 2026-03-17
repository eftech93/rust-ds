#!/bin/bash
# Create and run iOS simulator for the mobile app

echo "========================================="
echo "iOS Simulator Creator"
echo "========================================="
echo ""

# Get available device types
echo "Available iPhone device types:"
xcrun simctl list devicetypes 2>/dev/null | grep iPhone | head -10
echo ""

# Get available runtimes
echo "Available iOS runtimes:"
xcrun simctl list runtimes 2>/dev/null | grep iOS
echo ""

# Device name to create
DEVICE_NAME="${1:-iPhone 15 Test}"
DEVICE_TYPE="com.apple.CoreSimulator.SimDeviceType.iPhone-15"
RUNTIME="com.apple.CoreSimulator.SimRuntime.iOS-26-3"

echo "Creating simulator: $DEVICE_NAME"
echo "  Device Type: iPhone 15"
echo "  Runtime: iOS 26.3"
echo ""

# Create the device
UUID=$(xcrun simctl create "$DEVICE_NAME" "$DEVICE_TYPE" "$RUNTIME" 2>&1)

if [ $? -eq 0 ]; then
    echo "✅ Simulator created successfully!"
    echo "   UUID: $UUID"
    echo ""
    
    # Boot the device
    echo "📱 Booting simulator..."
    xcrun simctl boot "$UUID"
    
    # Open Simulator app
    echo "🖥️  Opening Simulator app..."
    open -a Simulator
    
    echo ""
    echo "✅ Simulator is ready!"
    echo ""
    echo "Next steps:"
    echo "  1. Build the app: cd examples/mobile && dx bundle --platform ios"
    echo "  2. Install app: xcrun simctl install booted ./target/dx/example-mobile/debug/ios/ExampleMobile.app"
    echo "  3. Run app: xcrun simctl launch booted com.example.ExampleMobile"
    echo ""
    echo "Or use the helper script:"
    echo "  ./examples/run-ios-sim.sh"
else
    echo "❌ Failed to create simulator"
    echo "Error: $UUID"
    echo ""
    echo "You may need to:"
    echo "  1. Install Xcode from App Store"
    echo "  2. Install iOS Simulator: Xcode → Preferences → Platforms → iOS"
fi

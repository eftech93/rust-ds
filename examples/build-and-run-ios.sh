#!/bin/bash
# Build and run iOS app on simulator

cd "$(dirname "$0")"

echo "========================================="
echo "iOS Build & Run"
echo "========================================="
echo ""

# Step 1: Build
echo "📦 Step 1: Building iOS app..."
echo "   This may take a few minutes..."
echo ""

cd mobile
dx bundle --platform ios

if [ $? -ne 0 ]; then
    echo ""
    echo "❌ Build failed!"
    exit 1
fi

echo ""
echo "✅ Build successful!"
echo ""

# Step 2: Run
echo "📱 Step 2: Running on simulator..."
echo ""

cd ..
./run-ios-sim.sh

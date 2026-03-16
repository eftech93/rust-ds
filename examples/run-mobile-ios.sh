#!/bin/bash
# Build and run Mobile example for iOS

cd "$(dirname "$0")/mobile"
echo "Building Mobile example for iOS..."
dx bundle --platform ios

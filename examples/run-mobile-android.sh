#!/bin/bash
# Build and run Mobile example for Android

cd "$(dirname "$0")/mobile"
echo "Building Mobile example for Android..."
dx bundle --platform android

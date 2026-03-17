#!/bin/bash
set -e

# Config
OUTPUT_DIR="/Users/tebo1993/Desktop/EFTECH93/rust-ds/docs/release"

echo "Building docs..."
dx build --platform web --release

echo "Copying to $OUTPUT_DIR..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"
cp -r /Users/tebo1993/Desktop/EFTECH93/rust-ds/target/dx/dioxus-ui-docs/release/web/public/* "$OUTPUT_DIR/"

echo "✓ Done! Output: $OUTPUT_DIR"
echo "  Test: cd release && python3 -m http.server 8000"

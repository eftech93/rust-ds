#!/bin/bash
set -e

# Get the workspace root (parent of docs-app)
WORKSPACE_ROOT="$(dirname "$(pwd)")"
OUTPUT_DIR="$WORKSPACE_ROOT/docs"

echo "Building docs..."
dx build --platform web --release

echo "Copying to $OUTPUT_DIR..."
rm -rf "$OUTPUT_DIR"/*
mkdir -p "$OUTPUT_DIR"
cp -r "$WORKSPACE_ROOT/target/dx/dioxus-ui-docs/release/web/public/"* "$OUTPUT_DIR/"

# Create 404.html for SPA routing (copy of index.html)
echo "Creating 404.html for SPA routing..."
cp "$OUTPUT_DIR/index.html" "$OUTPUT_DIR/404.html"

echo "✓ Done! Output: $OUTPUT_DIR"
echo "  Files:"
ls -la "$OUTPUT_DIR"

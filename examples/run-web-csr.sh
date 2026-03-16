#!/bin/bash
# Run Web Client-Side Rendering example

cd "$(dirname "$0")/web-csr"

# Check if dx is installed
if ! command -v dx &> /dev/null; then
    echo "❌ dx (Dioxus CLI) is not installed!"
    echo ""
    echo "To install it, run:"
    echo "  cargo install dioxus-cli@0.6.3"
    echo ""
    echo "For more details, see web-csr/README.md"
    exit 1
fi

echo "✅ dx found: $(dx --version)"
echo ""
echo "Starting Web CSR example..."
echo "Open http://localhost:8080 in your browser"
echo "Press Ctrl+C to stop"
echo ""

dx serve --platform web "$@"

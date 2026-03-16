#!/bin/bash
# Run Web Server-Side Rendering example

cd "$(dirname "$0")/web-ssr"
echo "Starting Web SSR example..."
echo "Open http://localhost:3000 in your browser"
cargo run

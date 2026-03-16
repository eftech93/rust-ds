#!/bin/bash
# Publish script for dioxus-ui-system

set -e

echo "========================================="
echo "Publishing dioxus-ui-system to crates.io"
echo "========================================="
echo ""

cd "$(dirname "$0")/../dioxus-ui-system"

# Check if logged in
echo "1. Checking cargo login status..."
cargo whoami 2>/dev/null || {
    echo "❌ Not logged in to crates.io"
    echo "   Run: cargo login"
    echo "   Get token from: https://crates.io/settings/tokens"
    exit 1
}
echo "✅ Logged in"
echo ""

# Verify build
echo "2. Verifying build..."
cargo build --release
echo "✅ Build successful"
echo ""

# Run tests
echo "3. Running tests..."
cargo test
echo "✅ Tests passed"
echo ""

# Generate docs
echo "4. Generating documentation..."
cargo doc --no-deps
echo "✅ Documentation generated"
echo ""

# Check package
echo "5. Checking package contents..."
cargo package --list | head -20
echo ""

# Dry run publish
echo "6. Dry run publish..."
cargo publish --dry-run --allow-dirty
echo "✅ Dry run successful"
echo ""

# Confirm
echo "========================================="
echo "Ready to publish!"
echo "========================================="
echo ""
echo "Version: $(grep '^version' Cargo.toml | head -1)"
echo ""
read -p "Are you sure you want to publish? (yes/no): " confirm

if [ "$confirm" = "yes" ]; then
    echo ""
    echo "Publishing..."
    cargo publish --allow-dirty
    echo ""
    echo "✅ Published successfully!"
    echo ""
    echo "View at: https://crates.io/crates/dioxus-ui-system"
    echo "Docs at: https://docs.rs/dioxus-ui-system"
else
    echo "Cancelled."
    exit 0
fi

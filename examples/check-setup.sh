#!/bin/bash
# Check development environment for all examples

echo "========================================="
echo "Dioxus UI Examples - Environment Check"
echo "========================================="
echo ""

cd "$(dirname "$0")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

check_command() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✅${NC} $2: $(command -v "$1")"
        return 0
    else
        echo -e "${RED}❌${NC} $2 not found"
        return 1
    fi
}

# Check Rust
echo "Checking Rust toolchain..."
check_command rustc "Rust"
check_command cargo "Cargo"
echo ""

# Check dx (Dioxus CLI)
echo "Checking Dioxus CLI (required for web & mobile)..."
if check_command dx "dx (Dioxus CLI)"; then
    dx --version
else
    echo -e "${YELLOW}⚠️${NC} Install with: cargo install dioxus-cli@0.6.3"
fi
echo ""

# Check which examples can be run
echo "========================================="
echo "Example Status"
echo "========================================="
echo ""

# Desktop
echo -n "Desktop Example: "
if check_command cargo "Cargo" > /dev/null; then
    echo -e "${GREEN}Ready${NC} - Run: ${GREEN}cargo run -p example-desktop${NC}"
else
    echo -e "${RED}Not available${NC}"
fi

# Web SSR
echo -n "Web SSR Example: "
if check_command cargo "Cargo" > /dev/null; then
    echo -e "${GREEN}Ready${NC} - Run: ${GREEN}cargo run -p example-web-ssr${NC}"
else
    echo -e "${RED}Not available${NC}"
fi

# Web CSR
echo -n "Web CSR Example: "
if check_command dx "dx" > /dev/null; then
    echo -e "${GREEN}Ready${NC} - Run: ${GREEN}./run-web-csr.sh${NC} or ${GREEN}dx serve --platform web${NC}"
else
    echo -e "${YELLOW}Need dx${NC} - Install: cargo install dioxus-cli@0.6.3"
fi

# Mobile
echo -n "Mobile Example: "
if check_command dx "dx" > /dev/null; then
    echo -e "${GREEN}Ready${NC} - Run: ${GREEN}./run-mobile-ios.sh${NC} or ${GREEN}./run-mobile-android.sh${NC}"
else
    echo -e "${YELLOW}Need dx${NC} - Install: cargo install dioxus-cli@0.6.3"
fi

echo ""
echo "========================================="
echo "Quick Start Commands"
echo "========================================="
echo ""
echo "Desktop:"
echo "  cargo run -p example-desktop"
echo ""
echo "Web SSR:"
echo "  cargo run -p example-web-ssr"
echo ""
echo "Web CSR (requires dx):"
echo "  ./run-web-csr.sh"
echo ""
echo "Mobile iOS (requires dx + macOS + Xcode):"
echo "  ./run-mobile-ios.sh"
echo ""
echo "Mobile Android (requires dx + Android SDK):"
echo "  ./run-mobile-android.sh"
echo ""

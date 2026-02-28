#!/bin/bash
# QB-COM Development Setup Script for Linux/macOS

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo ""
    echo "=========================================="
    echo "  $1"
    echo "=========================================="
    echo ""
}

print_header "QB-COM Development Setup"

# Check if Rust is installed
echo -e "${BLUE}Checking for Rust installation...${NC}"
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}[WARNING] Rust is not installed!${NC}"
    echo ""
    echo "Would you like to install Rust now? (y/n)"
    read -r response
    
    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo ""
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        echo -e "${GREEN}[OK] Rust installed. Please restart your terminal and run this script again.${NC}"
        exit 0
    else
        echo "Please install Rust from https://rustup.rs/ and run this setup again."
        exit 1
    fi
fi

echo -e "${GREEN}[OK] Rust found: $(rustc --version)${NC}"

# Check if cargo is working
echo -e "${BLUE}Checking Cargo...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}[ERROR] Cargo is not available!${NC}"
    echo "Please check your Rust installation."
    exit 1
fi
echo -e "${GREEN}[OK] Cargo is working: $(cargo --version)${NC}"

# Build the project
print_header "Building QB-COM"

echo -e "${YELLOW}Building in release mode...${NC}"
cargo build --release

if [[ $? -ne 0 ]]; then
    echo ""
    echo -e "${RED}[ERROR] Build failed!${NC}"
    echo "Please check the error messages above."
    exit 1
fi

echo -e "${GREEN}[OK] Build successful!${NC}"

# Run tests
print_header "Running Tests"

cargo test
if [[ $? -ne 0 ]]; then
    echo ""
    echo -e "${YELLOW}[WARNING] Some tests failed!${NC}"
    echo "You can still use the compiler, but some features may not work correctly."
    echo ""
fi

# Check if binary was created
BINARY="target/release/qb"
if [[ -f "$BINARY" ]]; then
    print_header "Setup Complete!"
    
    echo -e "${GREEN}QB-COM has been built successfully!${NC}"
    echo ""
    echo "Binary location: $BINARY"
    echo ""
    echo -e "${BLUE}Quick commands:${NC}"
    echo "  ./$BINARY --help              Show help"
    echo "  ./$BINARY run examples/hello.bas   Run example"
    echo "  ./$BINARY repl                Start interactive mode"
    echo ""
    echo -e "${BLUE}To install system-wide, run:${NC}"
    echo "  ./scripts/install.sh"
    echo ""
else
    echo -e "${RED}[ERROR] Could not find built binary at $BINARY${NC}"
    exit 1
fi

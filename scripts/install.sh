#!/bin/bash
# QB-COM Installation Script for Linux/macOS
# Usage: ./install.sh [options]
# Options:
#   --prefix PATH    Install prefix (default: /usr/local)
#   --user           Install to ~/.local instead
#   --no-examples    Don't install example files

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default settings
PREFIX="/usr/local"
USER_INSTALL=false
INSTALL_EXAMPLES=true
SKIP_BUILD=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --prefix)
            PREFIX="$2"
            shift 2
            ;;
        --user)
            USER_INSTALL=true
            PREFIX="$HOME/.local"
            shift
            ;;
        --no-examples)
            INSTALL_EXAMPLES=false
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        --help|-h)
            echo "QB-COM Installation Script"
            echo ""
            echo "Usage: ./install.sh [options]"
            echo ""
            echo "Options:"
            echo "  --prefix PATH    Install prefix (default: /usr/local)"
            echo "  --user           Install to ~/.local/bin"
            echo "  --no-examples    Don't install example files"
            echo "  --skip-build     Skip building (use existing binary)"
            echo "  --help, -h       Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Print header
echo ""
echo "=========================================="
echo "  QB-COM Installer"
echo "=========================================="
echo ""

# Check OS
OS=$(uname -s)
ARCH=$(uname -m)
echo -e "${BLUE}Detected: $OS $ARCH${NC}"

# Check prerequisites
echo ""
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}[ERROR] Rust/Cargo is not installed.${NC}"
    echo ""
    echo "Please install Rust first:"
    echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo -e "${GREEN}[OK] Cargo found: $(cargo --version)${NC}"

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
echo -e "${BLUE}Installing QB-COM version: $VERSION${NC}"

# Build if needed
if [[ "$SKIP_BUILD" == false ]]; then
    echo ""
    echo -e "${YELLOW}[1/4] Building QB-COM in release mode...${NC}"
    cargo build --release
    echo -e "${GREEN}[OK] Build successful${NC}"
else
    echo -e "${YELLOW}[1/4] Skipping build (using existing binary)${NC}"
fi

# Verify binary exists
BINARY="target/release/qb"
if [[ ! -f "$BINARY" ]]; then
    echo -e "${RED}[ERROR] Binary not found at $BINARY${NC}"
    exit 1
fi

# Create directories
echo ""
echo -e "${YELLOW}[2/4] Creating directories...${NC}"
mkdir -p "$PREFIX/bin"
mkdir -p "$PREFIX/share/qb-com/examples"
mkdir -p "$PREFIX/share/doc/qb-com"
echo -e "${GREEN}[OK] Directories created${NC}"

# Install binary
echo ""
echo -e "${YELLOW}[3/4] Installing binary to $PREFIX/bin/qb...${NC}"
cp "$BINARY" "$PREFIX/bin/qb"
chmod +x "$PREFIX/bin/qb"
echo -e "${GREEN}[OK] Binary installed${NC}"

# Install examples
echo ""
if [[ "$INSTALL_EXAMPLES" == true ]]; then
    echo -e "${YELLOW}[4/4] Installing example files...${NC}"
    if [[ -d "examples" ]]; then
        cp -r examples/* "$PREFIX/share/qb-com/examples/" 2>/dev/null || true
        echo -e "${GREEN}[OK] Examples installed to $PREFIX/share/qb-com/examples/${NC}"
    else
        echo -e "${YELLOW}[SKIP] No examples directory found${NC}"
    fi
else
    echo -e "${YELLOW}[4/4] Skipping examples installation${NC}"
fi

# Install documentation
echo ""
echo -e "${YELLOW}Installing documentation...${NC}"
[[ -f "LICENSE" ]] && cp LICENSE "$PREFIX/share/doc/qb-com/"
[[ -f "README.md" ]] && cp README.md "$PREFIX/share/doc/qb-com/"
echo -e "${GREEN}[OK] Documentation installed${NC}"

# Update PATH if needed
echo ""
if [[ "$USER_INSTALL" == true ]]; then
    SHELL_RC=""
    if [[ -n "$ZSH_VERSION" ]]; then
        SHELL_RC="$HOME/.zshrc"
    elif [[ -n "$BASH_VERSION" ]]; then
        SHELL_RC="$HOME/.bashrc"
    fi
    
    if [[ -n "$SHELL_RC" && -d "$HOME/.local/bin" ]]; then
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo -e "${YELLOW}Adding ~/.local/bin to PATH in $SHELL_RC${NC}"
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
            echo -e "${GREEN}[OK] PATH updated. Please restart your terminal or run: source $SHELL_RC${NC}"
        fi
    fi
fi

# Verify installation
echo ""
echo -e "${YELLOW}Verifying installation...${NC}"
if command -v qb &> /dev/null; then
    QB_VERSION=$(qb --version 2>/dev/null || echo "unknown")
    echo -e "${GREEN}[OK] QB-COM is installed and available!${NC}"
    echo -e "${BLUE}Version: $QB_VERSION${NC}"
else
    echo -e "${YELLOW}[WARNING] QB-COM installed but not in PATH${NC}"
    echo -e "${YELLOW}You may need to add $PREFIX/bin to your PATH${NC}"
fi

# Print summary
echo ""
echo "=========================================="
echo "  Installation Complete!"
echo "=========================================="
echo ""
echo "Installation directory: $PREFIX"
echo "Binary location: $PREFIX/bin/qb"
if [[ "$INSTALL_EXAMPLES" == true ]]; then
    echo "Examples: $PREFIX/share/qb-com/examples/"
fi
echo "Documentation: $PREFIX/share/doc/qb-com/"
echo ""
echo "Quick start:"
echo "    qb --help          Show help"
echo "    qb run file.bas    Run a QBasic program"
echo "    qb repl            Start interactive mode"
echo ""

# Uninstall instructions
echo "To uninstall:"
echo "    rm $PREFIX/bin/qb"
echo "    rm -rf $PREFIX/share/qb-com/"
echo ""

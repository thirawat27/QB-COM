#!/bin/bash
# QB-COM Uninstallation Script for Linux/macOS

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo ""
echo "=========================================="
echo "  QB-COM Uninstaller"
echo "=========================================="
echo ""

# Detect installation locations
LOCATIONS=(
    "/usr/local/bin/qb"
    "/usr/bin/qb"
    "$HOME/.local/bin/qb"
    "$HOME/.cargo/bin/qb"
)

SHARE_LOCATIONS=(
    "/usr/local/share/qb-com"
    "/usr/share/qb-com"
    "$HOME/.local/share/qb-com"
)

FOUND=false

# Check each location
for loc in "${LOCATIONS[@]}"; do
    if [[ -f "$loc" ]]; then
        echo -e "${BLUE}Found installation at: $loc${NC}"
        FOUND=true
        
        read -p "Remove this installation? [y/N] " -n 1 -r
        echo
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            # Remove binary
            if [[ -w "$(dirname "$loc")" ]]; then
                rm -f "$loc"
                echo -e "${GREEN}[OK] Removed: $loc${NC}"
            else
                echo -e "${YELLOW}Need sudo to remove $loc${NC}"
                sudo rm -f "$loc"
                echo -e "${GREEN}[OK] Removed: $loc${NC}"
            fi
            
            # Remove share directory
            for share in "${SHARE_LOCATIONS[@]}"; do
                if [[ -d "$share" ]]; then
                    if [[ -w "$share" ]] || [[ -w "$(dirname "$share")" ]]; then
                        rm -rf "$share"
                    else
                        sudo rm -rf "$share"
                    fi
                    echo -e "${GREEN}[OK] Removed: $share${NC}"
                fi
            done
            
            echo ""
            echo -e "${GREEN}QB-COM has been uninstalled successfully!${NC}"
        else
            echo -e "${YELLOW}Skipped.${NC}"
        fi
    fi
done

if [[ "$FOUND" == false ]]; then
    echo -e "${YELLOW}No QB-COM installation found.${NC}"
    echo ""
    echo "QB-COM might be installed in a custom location."
    echo "You can manually remove it with:"
    echo "    rm /path/to/qb"
fi

echo ""

#!/usr/bin/env bash
# vcpkg setup script for macOS/Linux
# This script installs all C/C++ dependencies via vcpkg for static linking

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default values
VCPKG_ROOT="${VCPKG_ROOT:-}"
TRIPLET="${1:-}"

# Detect OS and architecture for triplet
if [ -z "$TRIPLET" ]; then
    OS_TYPE=$(uname -s)
    ARCH=$(uname -m)

    case "$OS_TYPE" in
        Darwin)
            if [ "$ARCH" = "arm64" ]; then
                TRIPLET="arm64-osx"
            else
                TRIPLET="x64-osx"
            fi
            ;;
        Linux)
            if [ "$ARCH" = "aarch64" ]; then
                TRIPLET="arm64-linux"
            else
                TRIPLET="x64-linux"
            fi
            ;;
        *)
            echo -e "${RED}ERROR: Unsupported OS: $OS_TYPE${NC}"
            exit 1
            ;;
    esac
fi

echo -e "${CYAN}=== vcpkg Setup for Drop Compress Image ===${NC}"
echo ""

# Check if vcpkg is installed
if [ -z "$VCPKG_ROOT" ]; then
    echo -e "${RED}ERROR: VCPKG_ROOT environment variable is not set${NC}"
    echo -e "${YELLOW}Please install vcpkg and set VCPKG_ROOT:${NC}"
    echo -e "${YELLOW}  1. git clone https://github.com/Microsoft/vcpkg.git${NC}"
    echo -e "${YELLOW}  2. cd vcpkg && ./bootstrap-vcpkg.sh${NC}"
    echo -e "${YELLOW}  3. export VCPKG_ROOT=\$(pwd)${NC}"
    echo -e "${YELLOW}  4. Add 'export VCPKG_ROOT=/path/to/vcpkg' to ~/.bashrc or ~/.zshrc${NC}"
    exit 1
fi

VCPKG_EXE="$VCPKG_ROOT/vcpkg"
if [ ! -f "$VCPKG_EXE" ]; then
    echo -e "${RED}ERROR: vcpkg executable not found at $VCPKG_EXE${NC}"
    echo -e "${YELLOW}Please run bootstrap-vcpkg.sh first:${NC}"
    echo -e "${YELLOW}  cd $VCPKG_ROOT && ./bootstrap-vcpkg.sh${NC}"
    exit 1
fi

echo -e "${GREEN}Found vcpkg at: $VCPKG_EXE${NC}"
echo -e "${GREEN}Using triplet: $TRIPLET${NC}"
echo ""

# Install all C/C++ dependencies
PACKAGES=(
    "aom:$TRIPLET"                # libaom (AV1 encoder for AVIF)
    "libavif[aom]:$TRIPLET"       # libavif with aom codec
    "libjxl:$TRIPLET"             # libjxl (JPEG XL)
    "libwebp:$TRIPLET"            # libwebp (WebP codec)
    "openjpeg:$TRIPLET"           # OpenJPEG (JPEG 2000)
    "libjpeg-turbo:$TRIPLET"      # libjpeg-turbo (for jpegli)
    "lcms:$TRIPLET"               # Little CMS (color management)
)

for package in "${PACKAGES[@]}"; do
    echo -e "${CYAN}Installing $package...${NC}"

    if "$VCPKG_EXE" install "$package"; then
        echo -e "${GREEN}Successfully installed $package${NC}"
    else
        echo -e "${YELLOW}WARNING: Failed to install $package${NC}"
    fi
    echo ""
done

echo -e "${CYAN}=== Setup Complete ===${NC}"
echo -e "${GREEN}You can now build the project with:${NC}"
echo -e "${GREEN}  cargo build --release${NC}"
echo ""
echo -e "${YELLOW}Note: Make sure VCPKG_ROOT environment variable is set in your shell${NC}"
echo -e "${YELLOW}Add this to ~/.bashrc or ~/.zshrc:${NC}"
echo -e "${YELLOW}  export VCPKG_ROOT=$VCPKG_ROOT${NC}"

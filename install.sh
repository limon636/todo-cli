#!/bin/bash

# Todo CLI Installer Script
# Usage: curl -fsSL https://raw.githubusercontent.com/limon636/todo-cli/main/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Configuration
REPO_OWNER="limon636"
REPO_NAME="todo-cli"
BINARY_NAME="todo"
INSTALL_DIR="$HOME/.local/bin"

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BOLD}${BLUE}"
    echo "🦀 Todo CLI Installer"
    echo "===================="
    echo -e "${NC}"
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)
    
    case "$os" in
        Linux)
            OS="linux"
            ;;
        Darwin)
            OS="macos"
            ;;
        CYGWIN*|MINGW32*|MSYS*|MINGW*)
            OS="windows"
            ;;
        *)
            print_error "Unsupported operating system: $os"
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        armv7l)
            ARCH="armv7"
            ;;
        *)
            print_error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac
    
    print_status "Detected platform: $OS-$ARCH"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Rust if not present
install_rust() {
    if command_exists rustc; then
        print_status "Rust is already installed"
        return 0
    fi
    
    print_status "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_success "Rust installed successfully"
}

# Create install directory
create_install_dir() {
    if [ ! -d "$INSTALL_DIR" ]; then
        print_status "Creating install directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
    fi
}

# Add to PATH if not already there
update_path() {
    local shell_profile=""
    
    # Detect shell and set appropriate profile file
    case "$SHELL" in
        */bash)
            shell_profile="$HOME/.bashrc"
            ;;
        */zsh)
            shell_profile="$HOME/.zshrc"
            ;;
        */fish)
            shell_profile="$HOME/.config/fish/config.fish"
            ;;
        *)
            shell_profile="$HOME/.profile"
            ;;
    esac
    
    # Check if directory is already in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_status "Adding $INSTALL_DIR to PATH in $shell_profile"
        echo "" >> "$shell_profile"
        echo "# Todo CLI" >> "$shell_profile"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_profile"
        export PATH="$INSTALL_DIR:$PATH"
        print_success "Added $INSTALL_DIR to PATH"
    else
        print_status "$INSTALL_DIR is already in PATH"
    fi
}

# Try to download pre-built binary (if releases exist)
try_download_binary() {
    print_status "Checking for pre-built binary..."
    
    # Try to get latest release
    local latest_release_url="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
    local release_info
    
    if command_exists curl; then
        release_info=$(curl -s "$latest_release_url" 2>/dev/null || echo "")
    elif command_exists wget; then
        release_info=$(wget -qO- "$latest_release_url" 2>/dev/null || echo "")
    else
        print_warning "Neither curl nor wget found. Will compile from source."
        return 1
    fi
    
    if [ -z "$release_info" ] || echo "$release_info" | grep -q "Not Found"; then
        print_warning "No pre-built binaries available. Will compile from source."
        return 1
    fi
    
    # Extract download URL (this would need to be implemented based on your release naming)
    # For now, we'll fall back to source compilation
    print_warning "Pre-built binary download not implemented yet. Compiling from source."
    return 1
}

# Compile from source
compile_from_source() {
    print_status "Compiling todo CLI from source..."
    
    # Create temporary directory
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    # Clone repository
    print_status "Cloning repository..."
    if command_exists git; then
        git clone "https://github.com/$REPO_OWNER/$REPO_NAME.git"
        cd "$REPO_NAME"
    else
        print_status "Git not found. Downloading source archive..."
        if command_exists curl; then
            curl -L "https://github.com/$REPO_OWNER/$REPO_NAME/archive/main.tar.gz" | tar -xz
        elif command_exists wget; then
            wget -O- "https://github.com/$REPO_OWNER/$REPO_NAME/archive/main.tar.gz" | tar -xz
        else
            print_error "Cannot download source code. Please install git, curl, or wget."
            exit 1
        fi
        cd "$REPO_NAME-main"
    fi
    
    # Build release binary
    print_status "Building release binary... (this may take a few minutes)"
    cargo build --release
    
    # Copy binary to install directory
    cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"
    
    # Make executable
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    # Cleanup
    cd /
    rm -rf "$temp_dir"
    
    print_success "Compiled and installed todo CLI successfully"
}

# Verify installation
verify_installation() {
    if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
        print_success "Todo CLI installed successfully!"
        print_status "Binary location: $INSTALL_DIR/$BINARY_NAME"
        
        # Test if it's in PATH and working
        if command_exists todo; then
            local version_output=$(todo --version 2>/dev/null || echo "unknown")
            print_success "Installation verified: $version_output"
        else
            print_warning "Binary installed but not in current PATH"
            print_status "Please restart your terminal or run: export PATH=\"$INSTALL_DIR:\$PATH\""
        fi
        
        echo ""
        echo -e "${BOLD}${GREEN}🎉 Installation Complete!${NC}"
        echo ""
        echo -e "${BOLD}Quick Start:${NC}"
        echo "  todo add \"Learn Rust\"     # Add a task"
        echo "  todo list                  # List tasks"
        echo "  todo done 1                # Complete task"
        echo "  todo --help                # Show all commands"
        echo ""
        echo -e "${BOLD}TUI Mode:${NC}"
        echo "  todo tui                   # Interactive interface"
        echo ""
        echo -e "Documentation: ${BLUE}https://github.com/$REPO_OWNER/$REPO_NAME${NC}"
        
    else
        print_error "Installation failed. Binary not found at $INSTALL_DIR/$BINARY_NAME"
        exit 1
    fi
}

# Main installation process
main() {
    print_header
    
    # Check for required tools
    if ! command_exists curl && ! command_exists wget; then
        print_error "Either curl or wget is required for installation"
        exit 1
    fi
    
    detect_platform
    create_install_dir
    install_rust
    
    # Try binary download first, fall back to source compilation
    if ! try_download_binary; then
        compile_from_source
    fi
    
    update_path
    verify_installation
}

# Run main function
main "$@"
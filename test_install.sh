#!/bin/bash

# Test the install script locally
# Usage: ./test_install.sh

echo "🧪 Testing Todo CLI installer locally..."

# Make install script executable
chmod +x install.sh

# Run installer
./install.sh

echo ""
echo "🔍 Testing installation..."

# Test if todo command works
if command -v todo >/dev/null 2>&1; then
    echo "✅ todo command is available"
    echo "📍 Location: $(which todo)"
    echo "📦 Version: $(todo --version)"
    echo ""
    echo "🎯 Quick test:"
    todo add "Test task from installer"
    todo list
    echo ""
    echo "✅ Installation test completed successfully!"
else
    echo "❌ todo command not found in PATH"
    echo "📁 Check if it was installed to: $HOME/.local/bin/todo"
    exit 1
fi
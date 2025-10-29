#!/bin/bash

set -e

# Installation directory
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="gitp"
BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

# Create installation directory
mkdir -p "$INSTALL_DIR"

# Direct download URL
DOWNLOAD_URL="https://github.com/ayn0s/git-profile/releases/latest/download/gitp-linux-amd64"
echo "Using download URL: $DOWNLOAD_URL"

# Download the binary
echo "Downloading gitp..."
curl -L "$DOWNLOAD_URL" -o "$BINARY_PATH"
chmod +x "$BINARY_PATH"

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.zshrc" 2>/dev/null || true
    echo "Added $INSTALL_DIR to PATH in .bashrc and .zshrc (if it exists)"
fi

echo "gitp has been installed successfully!"
echo "You may need to restart your terminal or run 'source ~/.bashrc' for the 'gitp' command to be available."
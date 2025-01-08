#!/bin/bash

# Exit immediately if any command fails
set -e

# Name of the Rust binary
BINARY_NAME="ungit"

# GitHub repository URL for your tool
REPO_URL="https://github.com/CooperDActor-bytes/ungit.git"

# Temporary directory for downloading the source code
TEMP_DIR=$(mktemp -d)

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Clone the repository
echo "Cloning the repository..."
git clone "$REPO_URL" "$TEMP_DIR"

# Navigate to the cloned repository
cd "$TEMP_DIR"

# Build the project in release mode
echo "Building the project..."
cargo build --release

# Check if the binary exists
if [ ! -f "target/release/$BINARY_NAME" ]; then
    echo "Error: Build failed or binary not found in target/release/"
    exit 1
fi

# Move the binary to /usr/local/bin for global use
echo "Installing the binary to /usr/local/bin..."
sudo mv "target/release/$BINARY_NAME" /usr/local/bin/

# Set executable permissions (just in case)
sudo chmod +x /usr/local/bin/$BINARY_NAME

# Clean up the temporary directory
cd ~
rm -rf "$TEMP_DIR"

echo "Installation complete! You can now use '$BINARY_NAME' globally."


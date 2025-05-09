#!/bin/bash

set -e

BINARY_URL="https://github.com/theunrealtarik/fget/install.sh"
BINARY_NAME="fget"
DEST="/usr/bin/$BINARY_NAME"

echo "[*] Downloading binary from $BINARY_URL..."
curl -L "$BINARY_URL" -o "/tmp/$BINARY_NAME"

echo "[*] Making it executable..."
chmod +x "/tmp/$BINARY_NAME"

echo "[*] Moving to $DEST (requires sudo)..."
sudo mv "/tmp/$BINARY_NAME" "$DEST"

echo "[✓] Installed successfully at $DEST"

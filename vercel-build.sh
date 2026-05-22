#!/bin/bash
set -e

# Use Vercel's persistent cache directory
CACHE_DIR=".vercel/cache"
DX_CACHE="$CACHE_DIR/dx-cache"
CARGO_CACHE="$CACHE_DIR/cargo"

# Cache cargo registry + index between deploys
export CARGO_HOME="$CARGO_CACHE"
mkdir -p "$DX_CACHE/bin" "$CARGO_CACHE"
export PATH="$DX_CACHE/bin:$CARGO_HOME/bin:$PATH"

# Wasm target
rustup target add wasm32-unknown-unknown

# dx: use cached binary or compile once
if command -v dx >/dev/null 2>&1 && dx --version >/dev/null 2>&1; then
    echo "=== Using cached dx: $(dx --version) ==="
else
    echo "=== No cached dx found. Compiling (first deploy only)... ==="
    cargo install dioxus-cli --version 0.7.9 --locked --root "$DX_CACHE"
    echo "=== Installed: $(dx --version) ==="
fi

# Build app
echo "=== Building Dioxus Web App ==="
dx build --release --platform web -p web

echo "=== Build finished! ==="

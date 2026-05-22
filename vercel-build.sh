#!/bin/bash
set -e

CACHE_DIR=".vercel/cache"
DX_CACHE="$CACHE_DIR/dx-cache"
WASM_TARGET_CACHE="$CACHE_DIR/wasm-target"

export CARGO_HOME="$CACHE_DIR/cargo"
export PATH="$DX_CACHE/bin:$PATH"

rustup target add wasm32-unknown-unknown

# Install dx only if not cached
if ! command -v dx &> /dev/null; then
    echo "=== No cached dx found. Installing... ==="
    mkdir -p "$DX_CACHE/bin"
    cargo install dioxus-cli \
        --version 0.7.9 \
        --root "$DX_CACHE" \
        --locked \
        --debug   # <-- much faster, fine for a build tool
else
    echo "=== Using cached dx: $(dx --version) ==="
fi

# Restore wasm target cache
if [ -d "$WASM_TARGET_CACHE" ]; then
    echo "=== Restoring wasm target cache ==="
    cp -r "$WASM_TARGET_CACHE" target
fi

dx build --release --platform web

# Save wasm target cache (only the wasm-specific parts, not all of target/)
echo "=== Saving wasm target cache ==="
mkdir -p "$WASM_TARGET_CACHE"
# Cache only the wasm32 incremental build artifacts
if [ -d "target/wasm32-unknown-unknown" ]; then
    cp -r target/wasm32-unknown-unknown "$WASM_TARGET_CACHE/"
fi
